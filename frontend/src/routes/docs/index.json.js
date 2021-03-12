import send from "@polka/send";
import marked from "marked";
import fetch from "node-fetch";

let json;

export async function get(_req, res) {
    if (!json || process.env.NODE_ENV !== "production") {
        json = JSON.stringify(await generate());
    }

    send(res, 200, json, {
        "Content-Type": "application/json"
    });
}

async function fetch_markdown(path) {
    let response = await fetch(`https://raw.githubusercontent.com/feather-rs/book/master/src/${encodeURI(path)}.md`);
    let text = await response.text();
    return text
}

async function summary() {
    const summary_md = await fetch_markdown("SUMMARY");
    console.log(summary_md);

    const renderer = new marked.Renderer();

    const chapters = []
    let sections = [];

    renderer.list = (body, ordered, start) => {
        chapters.push(sections);
        sections = []
    };

    renderer.listitem = (text, task, checked) => {
        sections.push(text);
    }

    return {
        html: marked(summary_md, { renderer }),
        chapters,
    }
}

async function generate() {
    return {
        summary: await summary(),
        chapters: [
            {
                html: "",
                slug: "introduction",
                metadata: {
                    title: "Introduction"
                },
            }
        ]
    }
}
