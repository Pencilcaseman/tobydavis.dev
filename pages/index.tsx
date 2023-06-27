import type {NextPage} from 'next';
import {Nav} from '../components/navbar/navbar';
import {Layout} from '../components/navbar/layout';
import {Trusted} from '../components/trusted';
import {Box} from '../components/styles/box';
import {Features1} from '../components/features1';
import {Features2} from '../components/features2';
import {Features3} from '../components/features3';
import {Testimonials} from '../components/tesminonials';
import {Statistics} from '../components/statistics';
import {Plans} from '../components/plans';
import {Faq} from '../components/faq';
import {Trial} from '../components/trial';
import {Footer} from '../components/footer';
import {Flex} from "../components/styles/flex";
import {Button, Divider, Image, Input, Link, Text} from "@nextui-org/react";
import {CheckIcon} from "../components/icons/CheckIcon";
import React from "react";

import {ArticleCard} from "../components/article/card";

const Home: NextPage = () => {
    return (
        <Layout>
            <Nav/>
            <Box as="main">

                <Flex
                    css={{
                        "gap": "$3",
                        "px": "$6",
                        "flexDirection": "column",
                        "alignContent": "center",
                        "justifyContent": "center",
                        "alignItems": "center",
                        "width": "100%",
                        "@sm": {
                            flexDirection: "row",
                            mt: "$20",
                        },
                    }}
                    justify={"center"}
                >
                    <Box
                        css={{
                            pt: '$13',

                            display: 'flex',
                            flexDirection: 'column',
                            gap: '$5',
                        }}
                    >
                        <Box
                            css={{
                                maxWidth: "600px",
                            }}
                        >
                            <Text
                                h1
                                css={{
                                    display: "inline",
                                }}
                            >
                                {"Hi, I'm Toby Davis. I'm interested in "}
                            </Text>
                            <Text
                                h1
                                css={{
                                    display: 'inline',
                                }}
                                color="primary"
                            >
                                {"high performance computing"}
                            </Text>
                            <Text
                                h1
                                css={{
                                    display: 'inline',
                                }}
                            >
                                {" and "}
                            </Text>
                            <Text
                                h1
                                css={{
                                    display: 'inline',
                                }}
                                color="primary"
                            >
                                {"machine learning"}
                            </Text>
                            <Text
                                h1
                                css={{
                                    display: 'inline',
                                }}
                            >
                                {"."}
                            </Text>
                        </Box>

                        <Text
                            css={{
                                color: '$accents8',
                                maxWidth: '400px',
                            }}
                            size={'$lg'}
                            span
                        >
                            {"I'm a self-taught programmer with a passion for writing fast and efficient \
                            code. I am proficient in C++, Python, Javascript and many other languages."}
                        </Text>
                    </Box>
                    <Box
                        css={{
                            '& img': {
                                width: "775px",
                                objectFit: "contain",
                            }
                        }}
                    >
                        <img
                            src="https://cdn.midjourney.com/1b7db196-f85b-4ebc-8c8a-a7f00924cb0a/0_2.png"
                            alt="Image"
                            style={{
                                borderRadius: "50px",
                                width: "775px",
                                height: "450px",
                                objectFit: "cover",
                            }}
                        />
                    </Box>
                </Flex>

                <Flex
                    direction={"column"}
                    align={"center"}
                    justify={"center"}
                    css={{
                        gap: "$10",
                        pt: "$20",
                    }}
                >

                    <Text
                        h2
                        css={{
                            textColor: "red",
                            display: "inline",
                            fontWeight: "$semibold",
                        }}
                    >
                        Hello, World!
                    </Text>

                    <Flex
                        direction={"row"}
                        align={"center"}
                        justify={"center"}
                        wrap={"wrap"}
                        css={{
                            gap: "$10",
                            mw: "1500px"
                        }}
                        >

                        {
                            [
                                "Title 1",
                                "Title 2",
                                "Title 3",
                                "Title 4",
                                "Title 5",
                                "Title 6",
                                "Title 7",
                                "Title 8",
                                "Title 9",
                                "Title 10",
                            ].map((title) => (
                                <ArticleCard
                                    key={title}
                                    width={300}
                                    height={450}
                                    title={title}
                                    description={"Test Description but it's really long to test how the card wraps text and to see whether the little grey box expands to fit the description"}
                                    tags={[
                                        "LibRapid",
                                        "High Performance Computing",
                                    ]
                                    }
                                    image={"https://cdn.midjourney.com/57ec6897-7075-402a-93d1-0383de532eea/0_1.png"}
                                    link={"https://google.com"}
                                />
                            ))
                        }

                    </Flex>

                </Flex>

                <Flex
                    direction={"column"}
                    align={"center"}
                    css={{
                        pt: "$20",
                        gap: "$10",
                    }}
                >
                    <Flex
                        direction={"column"}
                        align={"center"}
                        css={{
                            gap: "$10",
                        }}
                    >
                        <Text
                            h2
                            css={{
                                display: "inline",
                                fontWeight: "$semibold",
                            }}
                        >
                            About Me
                        </Text>

                        <Box css={{
                            mw: "800px",
                        }}>
                            <Text
                                h3
                                css={{
                                    display: "inline",
                                    fontWeight: "$semibold",
                                }}
                            >
                                The Early Days
                            </Text>

                            <Text
                                css={{
                                    color: "$accents8",
                                    fontSize: "16pt"
                                }}
                            >
                                I first started programming in Scratch many years ago, where I made a few simple games
                                and animations. At some point I moved on to Python, where I learned to write more
                                complex programs. Wanting to visualise the things I made, I learned Processing and spent
                                a huge amount of time writing games, simulations, animations and more.
                            </Text>
                        </Box>

                        <Box css={{
                            mw: "800px",
                        }}>
                            <Text
                                h3
                                css={{
                                    display: "inline",
                                    fontWeight: "$semibold",
                                }}
                            >
                                Machine Learning?
                            </Text>

                            <Text
                                css={{
                                    color: "$accents8",
                                    fontSize: "16pt"
                                }}
                            >
                                At some point in my programming journey, I found
                                <Link
                                    href={"https://youtu.be/BOZfhUcNiqk"}
                                    css={{
                                        display: "inline",
                                    }}>
                                    {" a video "}
                                </Link>
                                by
                                <Link
                                    href={"https://www.youtube.com/@CodeBullet"}
                                    css={{
                                        display: "inline"
                                    }}>
                                    {" Code Bullet "}
                                </Link>
                                where he made a very simple genetic algorithm to teach dots how to find a target. After
                                watching the video repeatedly, I attempted to recreate the algorithms myself.
                            </Text>
                        </Box>

                        <Box css={{
                            mw: "800px",
                        }}>
                            <Text
                                css={{
                                    color: "$accents8",
                                    fontSize: "16pt"
                                }}
                            >
                                After a couple failed attempts, I finally managed to create my very first machine
                                learning program. That was the moment I fell in love with machine learning. I spent
                                countless hours learning about supervised and unsupervised learning methods, writing my
                                own neural network libraries and creating more advanced and powerful programs.
                            </Text>
                        </Box>

                        <Box css={{
                            mw: "800px",
                        }}>
                            <Text
                                css={{
                                    color: "$accents8",
                                    fontSize: "16pt"
                                }}
                            >
                                My favourite project was a simple self-driving car that I developed. A car would cast
                                rays out in multiple directions, receiving information about the distance to the nearest
                                wall. This information was passed into a neural network which determined whether the car
                                would accelerate, decelerate, turn left or turn right.
                            </Text>
                        </Box>

                        <Box css={{
                            mw: "800px",
                        }}>
                            <Text
                                css={{
                                    color: "$accents8",
                                    fontSize: "16pt"
                                }}
                            >
                                After a considerable amount of time waiting for the inefficient program to train, I was
                                left with a car that could drift around corners and complete the track perfectly.
                            </Text>
                        </Box>

                        <Box css={{
                            mw: "800px",
                        }}>
                            <Text
                                h3
                                css={{
                                    display: "inline",
                                    fontWeight: "$semibold",
                                }}
                            >
                                Speed and Power
                            </Text>

                            <Text
                                css={{
                                    color: "$accents8",
                                    fontSize: "16pt"
                                }}
                            >
                                Soon, I realised that the programs I was writing were too inefficient to be practical
                                for larger projects, so I started to learn C, which proved to be a bit more difficult
                                than I had anticipated. After a day or two of failing to fix countless bugs, I gave up
                                and moved on to C++, which I found significantly easier to learn.
                            </Text>
                        </Box>

                        <Box css={{
                            mw: "800px",
                        }}>
                            <Text
                                css={{
                                    color: "$accents8",
                                    fontSize: "16pt"
                                }}
                            >
                                Although I did eventually go back and learn C, C++ has become my favourite language to
                                work in and I have developed a very good understanding of it. I&apos;ve developed a
                                range of
                                libraries and programs in C++; most notably,

                                <Link
                                    href={"https://github.com/LibRapid/librapid"}
                                    css={{
                                        display: "inline",
                                    }}>
                                    {" LibRapid "}
                                </Link>

                                , which is an incredibly fast, open-source linear algebra library I have been working on
                                for a while now.
                            </Text>
                        </Box>
                    </Flex>
                </Flex>

                <Divider
                    css={{position: 'absolute', inset: '0p', left: '0', mt: '$10'}}
                />
                <Footer/>
            </Box>
        </Layout>
    );
};

export default Home;
