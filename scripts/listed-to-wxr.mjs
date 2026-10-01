// Converts the Listed RSS feed into a WordPress export (WXR) file for EmDash's
// built-in importer (/_emdash/admin/import/wordpress). The importer keeps the
// publish dates and slugs, converts the HTML to Portable Text, and copies the
// images listed as attachments into EmDash storage.
//
// Usage: node scripts/listed-to-wxr.mjs [feed-url-or-file] [out-dir]

import { readFile, writeFile, mkdir } from "node:fs/promises";
import { XMLParser } from "fast-xml-parser";
import { parseFragment, serializeOuter } from "parse5";

const source = process.argv[2] ?? "https://blog.lanesawyer.dev/feed";
const outDir = process.argv[3] ?? "import";
const AUTHOR = { login: "lane", displayName: "Lane Sawyer" };

const feedXml = source.startsWith("http")
	? await (await fetch(source)).text()
	: await readFile(source, "utf8");

const parser = new XMLParser({ processEntities: true, htmlEntities: true, parseTagValue: false });
const items = [parser.parse(feedXml).rss.channel.item].flat();

const attachments = new Map();
const redirects = {};
const unhandled = new Map();
let nextId = 1;

const usedSlugs = new Set();
// The feed is newest-first; reverse so the oldest post keeps a reused slug.
const posts = [...items].reverse().map((item) => {
	const link = new URL(item.link);
	const [listedId, listedSlug] = link.pathname.split("/").filter(Boolean);
	const date = new Date(item.pubDate);
	const slug = usedSlugs.has(listedSlug) ? `${listedSlug}-${date.getUTCFullYear()}` : listedSlug;
	usedSlugs.add(slug);
	const blocks = toBlocks(String(item.description));
	redirects[link.pathname] = `/posts/${slug}`;
	return {
		id: nextId++,
		listedId,
		title: String(item.title),
		slug,
		date,
		link: item.link,
		content: blocks.join("\n\n"),
		excerpt: excerptFrom(String(item.description)),
	};
});

await mkdir(outDir, { recursive: true });
await writeFile(`${outDir}/listed.wxr.xml`, toWxr(posts));
await writeFile(`${outDir}/redirects.json`, `${JSON.stringify(redirects, null, "\t")}\n`);

console.log(`posts: ${posts.length}, attachments: ${attachments.size}`);
if (unhandled.size) console.log("unhandled top-level tags (kept as HTML):", Object.fromEntries(unhandled));

function toBlocks(html) {
	const blocks = [];
	for (const node of parseFragment(html).childNodes) {
		const tag = node.tagName;
		if (!tag) {
			const text = node.value?.trim();
			if (text) blocks.push(block("paragraph", `<p>${escapeHtml(text)}</p>`));
			continue;
		}
		switch (tag) {
			case "p": {
				const children = node.childNodes.filter((c) => c.tagName || c.value?.trim());
				if (!children.length) break;
				if (findDescendant(node, "img")) {
					blocks.push(...splitImages(node));
					break;
				}
				blocks.push(block("paragraph", serializeOuter(node)));
				break;
			}
			case "h1":
			case "h2":
			case "h3":
			case "h4":
			case "h5":
			case "h6": {
				// The post title is the page's h1, so demote in-body headings one level.
				const level = Math.min(Number(tag[1]) + 1, 6);
				const inner = serializeOuter(node).replace(/^<h\d([^>]*)>|<\/h\d>$/g, "");
				blocks.push(block("heading", `<h${level}>${inner}</h${level}>`, { level }));
				break;
			}
			case "ul":
			case "ol":
				blocks.push(block("list", serializeOuter(node), tag === "ol" ? { ordered: true } : undefined));
				break;
			case "blockquote":
				blocks.push(block("quote", serializeOuter(node).replace("<blockquote", '<blockquote class="wp-block-quote"')));
				break;
			case "div":
			case "pre": {
				const pre = tag === "pre" ? node : findDescendant(node, "pre");
				if (!pre) {
					countUnhandled(tag);
					blocks.push(block("html", serializeOuter(node)));
					break;
				}
				const language = attr(pre, "class")
					?.split(/\s+/)
					.find((c) => c && c !== "highlight");
				const code = textContent(pre).replace(/^\n+|\s+$/g, "");
				const attrs = language && language !== "plaintext" ? { language } : undefined;
				blocks.push(block("code", `<pre class="wp-block-code"><code>${escapeHtml(code)}</code></pre>`, attrs));
				break;
			}
			case "hr":
				blocks.push(block("separator", '<hr class="wp-block-separator"/>'));
				break;
			case "img":
				blocks.push(imageBlock(node));
				break;
			case "br":
				break;
			default:
				countUnhandled(tag);
				blocks.push(block("html", serializeOuter(node)));
		}
	}
	return blocks;
}

// Listed renders images inline in paragraphs, often as `text<br><img><br>caption`.
// Split those into paragraph and image blocks, using text after an image as its caption.
function splitImages(p) {
	const blocks = [];
	let run = [];
	let lastImg;
	const flush = () => {
		const html = run.map(serializeOuter).join("").replace(/^(\s|<br>)+|(\s|<br>)+$/g, "");
		run = [];
		if (lastImg) blocks.push(imageBlock(lastImg, html || undefined));
		else if (html) blocks.push(block("paragraph", `<p>${html}</p>`));
		lastImg = undefined;
	};
	for (const child of p.childNodes) {
		if (child.tagName === "img") {
			flush();
			lastImg = child;
		} else {
			run.push(child);
		}
	}
	flush();
	return blocks;
}

function imageBlock(img, captionHtml) {
	const src = attr(img, "src");
	const alt = attr(img, "alt") ?? "";
	if (!attachments.has(src)) attachments.set(src, { id: 100000 + attachments.size, src });
	const caption = captionHtml ? `<figcaption class="wp-element-caption">${captionHtml}</figcaption>` : "";
	return block(
		"image",
		`<figure class="wp-block-image"><img src="${escapeAttr(src)}" alt="${escapeAttr(alt)}"/>${caption}</figure>`,
	);
}

function block(name, html, attrs) {
	const json = attrs ? ` ${JSON.stringify(attrs)}` : "";
	return `<!-- wp:${name}${json} -->\n${html}\n<!-- /wp:${name} -->`;
}

function excerptFrom(html) {
	const first = parseFragment(html).childNodes.find((n) => n.tagName === "p" && textContent(n).trim());
	const text = first ? textContent(first).replace(/\s+/g, " ").trim() : "";
	if (text.length <= 200) return text;
	return `${text.slice(0, 200).replace(/\s+\S*$/, "")}…`;
}

function toWxr(posts) {
	const gmt = (d) => d.toISOString().replace("T", " ").slice(0, 19);
	const postItems = posts.map(
		(p) => `
	<item>
		<title>${cdata(p.title)}</title>
		<link>${escapeHtml(p.link)}</link>
		<pubDate>${p.date.toUTCString()}</pubDate>
		<dc:creator>${cdata(AUTHOR.login)}</dc:creator>
		<guid isPermaLink="true">${escapeHtml(p.link)}</guid>
		<content:encoded>${cdata(p.content)}</content:encoded>
		<excerpt:encoded>${cdata(p.excerpt)}</excerpt:encoded>
		<wp:post_id>${p.id}</wp:post_id>
		<wp:post_date>${cdata(gmt(p.date))}</wp:post_date>
		<wp:post_date_gmt>${cdata(gmt(p.date))}</wp:post_date_gmt>
		<wp:post_name>${cdata(p.slug)}</wp:post_name>
		<wp:status>${cdata("publish")}</wp:status>
		<wp:post_type>${cdata("post")}</wp:post_type>
	</item>`,
	);
	const attachmentItems = [...attachments.values()].map(
		(a) => `
	<item>
		<title>${cdata(decodeURIComponent(a.src.split("/").pop()))}</title>
		<guid isPermaLink="false">${escapeHtml(a.src)}</guid>
		<wp:post_id>${a.id}</wp:post_id>
		<wp:status>${cdata("inherit")}</wp:status>
		<wp:post_type>${cdata("attachment")}</wp:post_type>
		<wp:attachment_url>${cdata(a.src)}</wp:attachment_url>
	</item>`,
	);
	return `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0"
	xmlns:excerpt="http://wordpress.org/export/1.2/excerpt/"
	xmlns:content="http://purl.org/rss/1.0/modules/content/"
	xmlns:dc="http://purl.org/dc/elements/1.1/"
	xmlns:wp="http://wordpress.org/export/1.2/">
<channel>
	<title>Lane Sawyer</title>
	<link>https://lanesawyer.dev</link>
	<language>en</language>
	<wp:wxr_version>1.2</wp:wxr_version>
	<wp:base_site_url>https://lanesawyer.dev</wp:base_site_url>
	<wp:base_blog_url>https://lanesawyer.dev</wp:base_blog_url>
	<wp:author>
		<wp:author_id>1</wp:author_id>
		<wp:author_login>${cdata(AUTHOR.login)}</wp:author_login>
		<wp:author_display_name>${cdata(AUTHOR.displayName)}</wp:author_display_name>
	</wp:author>
${[...attachmentItems, ...postItems].join("")}
</channel>
</rss>
`;
}

function findDescendant(node, tag) {
	for (const child of node.childNodes ?? []) {
		if (child.tagName === tag) return child;
		const found = findDescendant(child, tag);
		if (found) return found;
	}
}

function textContent(node) {
	if (node.nodeName === "#text") return node.value;
	return (node.childNodes ?? []).map(textContent).join("");
}

function attr(node, name) {
	return node.attrs?.find((a) => a.name === name)?.value;
}

function countUnhandled(tag) {
	unhandled.set(tag, (unhandled.get(tag) ?? 0) + 1);
}

function cdata(s) {
	return `<![CDATA[${String(s).replaceAll("]]>", "]]]]><![CDATA[>")}]]>`;
}

function escapeHtml(s) {
	return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

function escapeAttr(s) {
	return escapeHtml(s).replace(/"/g, "&quot;");
}
