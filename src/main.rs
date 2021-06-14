#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Project<'s> {
    link: &'s str,
    name: &'s str,
    description: &'s str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct NavLink<'s> {
    link: &'s str,
    svg: &'s str,
    name: &'s str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Context<'s> {
    description: &'s str,
    projects: Vec<Project<'s>>,
    nav_links: Vec<NavLink<'s>>,
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "page",
        Context {
            description: "embedded development, hardware",
            projects: vec![
                Project {
                    link: "https://github.com/andi-makes/aWristWatch",
                    name: "aWristWatch",
                    description: "my own wristwatch design",
                },
                Project {
                    link: "https://github.com/andi-makes/rusty_im",
                    name: "Rusty Inventory Manager",
                    description: "a simple way to manage your stuff",
                },
                Project {
                    link: "https://github.com/andi-makes/aoc2020",
                    name: "Advent of Code",
                    description: "trying to solve as many puzzles as I can since 2019",
                },
            ],
            nav_links: vec![
                NavLink {
                    link: "https://github.com/andi-makes",
                    svg: "<path d=\"M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z\" />",
                    name: "GitHub"
                },
                NavLink {
                    link: "https://www.twitch.tv/andi_makes",
                    svg: r#"<path d="M3.857 0 1 2.857v10.286h3.429V16l2.857-2.857H9.57L14.714 8V0H3.857zm9.714 7.429-2.285 2.285H9l-2 2v-2H4.429V1.143h9.142v6.286z" />
                            <path d="M11.857 3.143h-1.143V6.57h1.143V3.143zm-3.143 0H7.571V6.57h1.143V3.143z" />"#,
                    name: "Twitch"
                },
                NavLink {
                    link: "https://discord.gg/yENgYZdmKY",
                    svg: r#"<path d="M6.552 6.712c-.456 0-.816.4-.816.888s.368.888.816.888c.456 0 .816-.4.816-.888.008-.488-.36-.888-.816-.888zm2.92 0c-.456 0-.816.4-.816.888s.368.888.816.888c.456 0 .816-.4.816-.888s-.36-.888-.816-.888z" />
                            <path d="M13.36 0H2.64C1.736 0 1 .736 1 1.648v10.816c0 .912.736 1.648 1.64 1.648h9.072l-.424-1.48 1.024.952.968.896L15 16V1.648C15 .736 14.264 0 13.36 0zm-3.088 10.448s-.288-.344-.528-.648c1.048-.296 1.448-.952 1.448-.952-.328.216-.64.368-.92.472-.4.168-.784.28-1.16.344a5.604 5.604 0 0 1-2.072-.008 6.716 6.716 0 0 1-1.176-.344 4.688 4.688 0 0 1-.584-.272c-.024-.016-.048-.024-.072-.04-.016-.008-.024-.016-.032-.024-.144-.08-.224-.136-.224-.136s.384.64 1.4.944c-.24.304-.536.664-.536.664-1.768-.056-2.44-1.216-2.44-1.216 0-2.576 1.152-4.664 1.152-4.664 1.152-.864 2.248-.84 2.248-.84l.08.096c-1.44.416-2.104 1.048-2.104 1.048s.176-.096.472-.232c.856-.376 1.536-.48 1.816-.504.048-.008.088-.016.136-.016a6.521 6.521 0 0 1 4.024.752s-.632-.6-1.992-1.016l.112-.128s1.096-.024 2.248.84c0 0 1.152 2.088 1.152 4.664 0 0-.68 1.16-2.448 1.216z" />"#,
                    name: "Discord"
                },
            ]
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from("./static"))
        .attach(Template::fairing())
}
