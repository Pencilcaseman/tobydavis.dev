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
import {Button, Divider, Image, Input, Text} from "@nextui-org/react";
import {CheckIcon} from "../components/icons/CheckIcon";
import React from "react";

const Home: NextPage = () => {
    return (
        <Layout>
            <Nav/>
            <Box as="main">

                <Flex
                    css={{
                        'gap': '$3',
                        'px': '$6',
                        'flexDirection': 'column',
                        'alignContent': 'center',
                        'justifyContent': 'center',
                        'alignItems': 'center',
                        'width': '100%',
                        '@sm': {
                            flexDirection: 'row',
                            mt: '$20',
                        },
                    }}
                    justify={'center'}
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
                                maxWidth: '600px',
                            }}
                        >
                            <Text
                                h1
                                css={{
                                    display: 'inline',
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
                                {"machine learning."}
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
                <Divider
                    css={{position: 'absolute', inset: '0p', left: '0', mt: '$10'}}
                />
                <Footer/>
            </Box>
        </Layout>
    );
};

export default Home;
