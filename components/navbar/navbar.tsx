import {Button, Dropdown, Link, Navbar, Switch, Text, Image} from '@nextui-org/react';
import React from 'react';
import {useRouter} from "next/router";
import {useState} from "react";
import {ModalLogin} from '../modal';
import {icons} from './icons';
import {AcmeLogo} from './logo';
import {useTheme as useNextTheme} from 'next-themes';
import {useTheme} from '@nextui-org/react';
import {GithubIcon} from '../icons/GithubIcon';

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

function navbarDropdownMenu(label: string, elements: DropdownElement[]) {
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
                        dflex: 'center',
                        svg: {pe: 'none'},
                    }}
                    iconRight={icons.chevron}
                    ripple={false}
                >
                    {label}
                </Dropdown.Button>
            </Navbar.Item>
            <Dropdown.Menu
                aria-label={label}
                css={{
                    "$$dropdownMenuWidth": "340px",
                    "$$dropdownItemHeight": "70px",
                    "& .nextui-dropdown-item": {
                        "py": '$4',
                        "svg": {
                            color: "$secondary",
                            mr: "$4",
                        },
                        "& .nextui-dropdown-item-content": {
                            w: '100%',
                            fontWeight: "$semibold",
                        },
                    },
                }}
                onAction={(item) => {
                    elements.forEach((element) => {
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
                {elements.map((element) => (
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

export const Nav = () => {
    const {setTheme} = useNextTheme();
    const {isDark, type} = useTheme();
    const collapseItems = [
        'Features',
        'Customers',
        'Pricing',
        'Company',
        'Legal',
    ];
    return (
        <Navbar
            isBordered
            css={{
                'overflow': 'hidden',
                '& .nextui-navbar-container': {
                    background: '$background',
                    borderBottom: 'none',
                },
            }}
        >
            <Navbar.Brand>
                <Navbar.Toggle aria-label="toggle navigation" showIn="xs"/>
                <AcmeLogo/>
                <Text b color="inherit" hideIn="xs">
                    Toby Davis
                </Text>
                <Navbar.Content
                    hideIn="sm"
                    css={{
                        pl: '6rem',
                    }}
                >
                    {navbarDropdownMenu("Projects", projects)}
                    {navbarDropdownMenu("Tutorials", tutorials)}
                    {navbarDropdownMenu("Socials", socials)}

                    <Navbar.Link isActive href="#">
                        Customers
                    </Navbar.Link>
                    <Navbar.Link href="#">Pricing</Navbar.Link>
                    <Navbar.Link href="#">Company</Navbar.Link>
                </Navbar.Content>
            </Navbar.Brand>

            <Navbar.Collapse>
                {collapseItems.map((item, index) => (
                    <Navbar.CollapseItem key={item}>
                        <Link
                            color="inherit"
                            css={{
                                minWidth: '100%',
                            }}
                            href="#"
                        >
                            {item}
                        </Link>
                    </Navbar.CollapseItem>
                ))}
                <Navbar.CollapseItem>
                    <Link
                        color="inherit"
                        css={{
                            minWidth: '100%',
                        }}
                        target="_blank"
                        href="https://github.com/Siumauricio/landing-template-nextui"
                    >
                        <GithubIcon/>
                    </Link>
                </Navbar.CollapseItem>
                <Navbar.CollapseItem>
                    <Switch
                        checked={isDark}
                        onChange={(e) =>
                            setTheme(e.target.checked ? 'dark' : 'light')
                        }
                    />
                </Navbar.CollapseItem>
            </Navbar.Collapse>
            <Navbar.Content>
                <ModalLogin/>

                <Navbar.Item>
                    <Button auto flat href="#">
                        Start free trial
                    </Button>
                </Navbar.Item>
                <Navbar.Item hideIn={'xs'}>
                    <Link
                        color="inherit"
                        css={{
                            minWidth: '100%',
                        }}
                        target="_blank"
                        href="https://github.com/Siumauricio/landing-template-nextui"
                    >
                        <GithubIcon/>
                    </Link>
                </Navbar.Item>
                <Navbar.Item hideIn={'xs'}>
                    <Switch
                        checked={isDark}
                        onChange={(e) =>
                            setTheme(e.target.checked ? 'dark' : 'light')
                        }
                    />
                </Navbar.Item>
            </Navbar.Content>
        </Navbar>
    );
};
