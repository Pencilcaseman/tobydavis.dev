import {Button, Dropdown, Link, Navbar, Switch, Text, Image} from "@nextui-org/react";
import React from "react";
import {useRouter} from "next/router";
import {useState} from "react";
import {ModalLogin} from "../modal";
import {icons} from "../icons/icons";
import {PenciLogo} from "../icons/logo";
import {useTheme as useNextTheme} from "next-themes";
import {useTheme} from "@nextui-org/react";
import {GithubIcon} from "../icons/GithubIcon";

interface DropdownElement {
    key: string;
    title: string;
    description: string;
    icon: React.ReactNode;
    link?: string;
}

const projects = [
    {
        key: "librapid",
        title: "LibRapid",
        description: "A high-performance C++ library for mathematics and linear algebra.",
        icon: icons.flash,
        link: "/projects/librapid"
    },
    {
        key: "symbomath",
        title: "SymboMath",
        description: "A fast and powerful symbolic mathematics library and research project.",
        icon: icons.rootX,
        link: "/projects/symbomath"
    },
    {
        key: "surge",
        title: "Surge",
        description: "A simple C++ graphics library for creative coding.",
        icon: icons.scale,
        link: "/projects/surge"
    },
    {
        key: "machineLearning",
        title: "Machine Learning",
        description: "Experiments with machine learning and artificial intelligence.",
        icon: icons.server,
        link: "/projects/machineLearning"
    },
]

const socials = [
    {
        key: "github",
        title: "GitHub",
        description: "My GitHub profile with all my projects and source code.",
        icon: icons.github,
        link: "https://github.com/Pencilcaseman/"
    },
    {
        key: "discord",
        title: "Discord",
        description: "The LibRapid Discord server, where you can talk about anything programming related.",
        icon: icons.discord,
        link: "https://discord.gg/cGxTFTgCAC"
    },
    {
        key: "devto",
        title: "Dev.to",
        description: "My Dev.to profile with some articles and tutorials.",
        icon: icons.devTo,
        link: "https://dev.to/pencilcaseman/"
    },
    {
        key: "linkedin",
        title: "LinkedIn",
        description: "My LinkedIn profile with my work experience and education.",
        icon: icons.linkedIn,
        link: "https://uk.linkedin.com/in/toby-davis-codes/"
    },
    {
        key: "stackoverflow",
        title: "Stack Overflow",
        description: "My Stack Overflow profile with my questions and answers.",
        icon: icons.stackOverflow,
        link: "https://stackoverflow.com/users/11564403/pencilcaseman"
    },
    {
        key: "midjourney",
        title: "Midjourney",
        description: "My Midjourney profile with AI-generated images, some of which are used on this website.",
        icon: icons.midjourney,
        link: "https://www.midjourney.com/app/users/84b4ba6f-22f0-4339-9812-dd1382ef4d9b/"
    }
]

const tutorials: DropdownElement[] = [
    {
        key: "example",
        title: "example",
        description: "example",
        icon: icons.flash,
        link: "https://github.com/Pencilcaseman/"
    },
];

interface NavbarDropdownMenuProps {
    label: string;
    elements: DropdownElement[];
}

function NavbarDropdownMenu(props: NavbarDropdownMenuProps) {
    let router = useRouter();
    const [selectedKey, setSelectedKey] = useState();

    return (
        <Dropdown isBordered>
            <Navbar.Item>
                <Dropdown.Button
                    auto
                    light
                    css={{
                        px: 0,
                        dflex: "center",
                        svg: {pe: "none"},
                    }}
                    iconRight={icons.chevron}
                    ripple={false}
                >
                    {props.label}
                </Dropdown.Button>
            </Navbar.Item>
            <Dropdown.Menu
                aria-label={props.label}
                css={{
                    "$$dropdownMenuWidth": "340px",
                    "$$dropdownItemHeight": "70px",
                    "& .nextui-dropdown-item": {
                        "py": "$4",
                        "svg": {
                            color: "$secondary",
                            mr: "$4",
                        },
                        "& .nextui-dropdown-item-content": {
                            w: "100%",
                            fontWeight: "$semibold",
                        },
                    },
                }}
                onAction={(item) => {
                    props.elements.forEach((element) => {
                        if (element.key == item) {
                            if (element.link) {
                                if (element.link.startsWith("http")) {
                                    window.open(element.link, "_blank");
                                } else {
                                    router.push(element.link).catch((err) => {
                                        console.log(err);
                                    });
                                }
                            }
                        }
                    })
                }}
            >
                {props.elements.map((element) => (
                    <Dropdown.Item
                        key={element.key}
                        showFullDescription
                        description={element.description}
                        icon={element.icon}
                    >
                        {element.title}
                    </Dropdown.Item>
                ))}
            </Dropdown.Menu>
        </Dropdown>
    )
}

function NavbarCollapseMenu(props: NavbarDropdownMenuProps) {
    return (
        <>
            <Navbar.CollapseItem>
                <Text
                    color="$primary"
                    css={{
                        fontWeight: "$semibold",
                        fontSize: "18pt",
                    }}
                >
                    {props.label}
                </Text>
            </Navbar.CollapseItem>

            {props.elements.map((element) => (
                <Navbar.CollapseItem key={element.title}>
                    <Link
                        color="inherit"
                        css={{
                            minWidth: "100%",
                        }}
                        href={element.link}
                    >
                        {"--> " + element.title}
                    </Link>
                </Navbar.CollapseItem>
            ))}

        </>
    )
}

export const Nav = () => {
    const {setTheme} = useNextTheme();
    const {isDark, type} = useTheme();
    const collapseItems = [
        "Projects",
        "Tutorials",
        "Articles",
        "Photography",
        "Socials",
    ];
    return (
        <Navbar
            isBordered
            css={{
                "overflow": "hidden",
                "& .nextui-navbar-container": {
                    background: "$background",
                    borderBottom: "none",
                },
            }}
        >
            <Navbar.Brand>
                <Navbar.Toggle aria-label="toggle navigation" showIn="xs"/>
                {PenciLogo()}
                <Text b color="inherit" hideIn="xs">
                    Toby Davis
                </Text>
                <Navbar.Content
                    hideIn="sm"
                    css={{
                        pl: "6rem",
                    }}
                >

                    <NavbarDropdownMenu label={"Projects"} elements={projects}/>
                    <NavbarDropdownMenu label={"Tutorials"} elements={tutorials}/>
                    <NavbarDropdownMenu label={"Socials"} elements={socials}/>

                </Navbar.Content>
            </Navbar.Brand>

            <Navbar.Collapse transitionTime={0} showIn={"xs"}>
                <Navbar.CollapseItem>
                    <Switch
                        checked={isDark}
                        onChange={(e) =>
                            setTheme(e.target.checked ? "dark" : "light")
                        }
                    />
                </Navbar.CollapseItem>

                <NavbarCollapseMenu label={"Projects"} elements={projects}/>
                <NavbarCollapseMenu label={"Tutorials"} elements={tutorials}/>
                <NavbarCollapseMenu label={"Socials"} elements={socials}/>
            </Navbar.Collapse>

            <Navbar.Content>
                <Navbar.Item hideIn={"xs"}>
                    <Switch
                        checked={isDark}
                        onChange={(e) =>
                            setTheme(e.target.checked ? "dark" : "light")
                        }
                    />
                </Navbar.Item>
            </Navbar.Content>
        </Navbar>
    );
};
