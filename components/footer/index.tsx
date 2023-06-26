import {Divider, Link, Text} from "@nextui-org/react";
import React from "react";
import {PenciLogo} from "../icons/logo";
import {Box} from "../styles/box";
import {Flex} from "../styles/flex";
import {icons} from "../icons/icons";

interface FooterItem {
    key: string;
    title: string;
    link: string;
}

function footerList(title: string, items: FooterItem[]) {
    return (
        <Flex
            direction={"column"}
            css={{
                "gap": "$4",
            }}>

            <Text
                css={{
                    "color": "$primary",
                    "fontWeight": "$semibold",
                    "fontSize": "18pt",
                }}
            >
                {title}
            </Text>
            {
                items.map((item) => (
                    <Link
                        key={item.key}
                        href={item.link}
                        css={{
                            "color": "$accents8",
                            "&:hover": {
                                color: "$accents9",
                            },
                        }}
                    >
                        {item.title}
                    </Link>
                ))
            }
        </Flex>
    )
}

export const Footer = () => {
    return (
        <Flex
            css={{
                py: "$20",
                px: "$6",
            }}
        >
            <Box as={"footer"} css={{width: "100%"}}>
                <Flex
                    direction={"row"}
                    justify={"center"}
                    align={"center"}
                    wrap={"wrap"}
                    css={{
                        "gap": "$40",
                        "pt": "$2",
                        "pb": "$10"
                    }}
                >
                    {footerList("Links", [
                        {
                            key: "librapidGitHub",
                            title: "LibRapid GitHub",
                            link: "https://github.com/LibRapid/librapid/"
                        },
                        {
                            key: "discord",
                            title: "LibRapid Discord",
                            link: "https://discord.gg/cGxTFTgCAC"
                        },
                        {
                            key: "devto",
                            title: "Dev.to",
                            link: "https://dev.to/pencilcaseman"
                        },
                        {
                            key: "stackoverflow",
                            title: "Stack Overflow",
                            link: "https://stackoverflow.com/users/11564403/pencilcaseman"
                        }
                    ])
                    }

                    {footerList("Contact", [
                        {
                            key: "email",
                            title: "Email",
                            link: "mailto:pencilcaseman@gmail.com"
                        },
                        {
                            key: "reddit",
                            title: "Reddit",
                            link: "https://www.reddit.com/user/Pencilcaseman12"
                        },
                        {
                            key: "discord",
                            title: "Discord",
                            link: "https://discordapp.com/users/539900920763252743"
                        },
                        {
                            key: "linkedin",
                            title: "LinkedIn",
                            link: "https://uk.linkedin.com/in/toby-davis-codes/"
                        }
                    ])
                    }
                </Flex>
                <Box
                    css={{
                        "px": "$10",
                        "@md": {
                            px: "$56",
                        },
                    }}
                >
                    <Divider
                        css={{
                            mt: "$14",
                            display: "flex",
                            justifyContent: "center",
                        }}
                    />
                    <Flex
                        //   justify={"between"}
                        align={"center"}
                        wrap={"wrap"}
                        css={{
                            "pt": "$8",
                            "gap": "$10",
                            "justifyContent": "center",
                            "@md": {
                                justifyContent: "space-between",
                            },
                        }}
                    >
                        <Flex
                            css={{
                                gap: "$10",
                            }}
                            wrap={"wrap"}
                        >
                            {PenciLogo()}
                        </Flex>
                        <Flex
                            css={{
                                gap: "$6",
                            }}
                        >
                            <Text span css={{color: "$accents8"}}>
                                © Copyright {new Date().getFullYear().toString()} Toby Davis.
                            </Text>
                        </Flex>
                    </Flex>
                </Box>
            </Box>
        </Flex>
    )
        ;
};
