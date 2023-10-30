import type {NextPage} from 'next';
import {Nav} from '../../components/navbar/navbar';
import {Layout} from '../../components/navbar/layout';
import {Box} from '../../components/styles/box';
import {Footer} from '../../components/footer';
import {Flex} from "../../components/styles/flex";
import {Button, Divider, Image, Input, Link, Text, useTheme} from "@nextui-org/react";
import React from "react";
import {PageTag} from "../../components/meta/pageTag";
import {ArticleCard} from "../../components/article/card";
import {useTheme as useNextTheme} from "next-themes";

const tutorials = [
    {
        key: "machineLearning",
        width: 300,
        height: 450,
        title: "Machine Learning",
        description: "Tutorials and guides on machine learning and AI.",
        tags: [
            "Machine Learning",
            "AI",
        ],
        image: "/images/neuralNetworkDiagramLong.png",
        textColor: "#142f5d",
        link: "/tutorials/machineLearning/machineLearning"
    },
]

const Home: NextPage = () => {
    const {setTheme} = useNextTheme();
    const {isDark, type} = useTheme();

    return (
        <Layout>
            <PageTag pageTitle={"Toby Davis - Tutorials"}
                     contentTitle={"Tutorials, Guides and Articles"}
                     description={"My tutorials, guides and articles on various topics."}
                     url={"https://tobydavis.dev/tutorials/tutorials"}
            />

            <Nav/>

            <Box as="main" css={{
                height: "100%",
                overflow: "auto"
            }}>
                <Flex
                    direction={"column"}
                    align={"center"}
                    justify={"center"}
                    css={{
                        gap: "$10",
                        pt: "$20",
                        height: "70vh"
                    }}
                >
                    <Flex css={{
                        flexDirection: "row",
                        justifyContent: "center",
                        height: "80vh",
                    }}>
                        <h1>Tutorials</h1>
                    </Flex>

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
                            tutorials.map((tutorial) => (
                                <ArticleCard
                                    key={tutorial.key}
                                    width={tutorial.width}
                                    height={tutorial.height}
                                    title={tutorial.title}
                                    textColour={tutorial.textColor}
                                    description={tutorial.description}
                                    tags={tutorial.tags}
                                    image={tutorial.image}
                                    link={tutorial.link}
                                />
                            ))
                        }
                    </Flex>
                </Flex>

                <Divider
                    css={{position: 'absolute', inset: '0p', left: '0', mt: '$10'}}
                />
                <Footer/>
            </Box>
        </Layout>
    )
        ;
};

export default Home;
