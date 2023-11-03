import type {NextPage} from 'next';
import {Nav} from '../../../components/navbar/navbar';
import {Layout} from '../../../components/navbar/layout';
import {Box} from '../../../components/styles/box';
import {Footer} from '../../../components/footer';
import {Flex} from "../../../components/styles/flex";
import {Button, Divider, Image, Input, Link, Text, useTheme} from "@nextui-org/react";
import React from "react";
import {PageTag} from "../../../components/meta/pageTag";
import {ArticleCard} from "../../../components/article/card";
import {useTheme as useNextTheme} from "next-themes";

const mlTutorials = [
    {
        key: "mlincpp",
        width: 300,
        height: 450,
        title: "Exploring Neural Networks in C++",
        description: "Learn fundamental concepts about machine learning and how to implement them in C++.",
        tags: [
            "Machine Learning",
            "C++",
        ],
        image: "/images/neuralNetworkDiagramLong.png",
        textColor: "#142f5d",
        link: "/tutorials/machineLearning/exploringNeuralNetworksInCPP"
    },
]

const Home: NextPage = () => {
    const {setTheme} = useNextTheme();
    const {isDark, type} = useTheme();

    return (
        <Layout>
            <PageTag pageTitle={"Toby Davis - Machine Learning"}
                     contentTitle={"Machine Learning Tutorials"}
                     description={"Tutorials and guides on machine learning and AI."}
                     url={"https://tobydavis.dev/tutorials/machineLearning/machineLearning"}
            />

            <Nav/>

            <Box as="main" css={{
                height: "100%",
                overflow: "auto"
            }}>
                <Flex css={{
                    flexDirection: "row",
                    alignItems: "center",
                    justifyContent: "center",
                    pt: "$5",
                }}>
                    <Flex css={{
                        flexDirection: "column",
                        alignItems: "center",
                        justifyContent: "center",
                        pt: "$5",
                        pl: "$12",
                        pr: "$12",
                        maxWidth: "800px",
                    }}>
                        <Text h1 id={"highPerformance"}>Machine Learning Tutorials</Text>

                        <Text css={{
                            fontSize: "16pt"
                        }}>
                            This page contains my tutorials and guides on machine learning and AI. Most of the code will
                            be written in C++, but the concepts can be applied to any language. I will primarily be
                            using <a href={"/librapid/librapid"}>LibRapid</a> for linear algebra and other mathematical
                            operations, and will use <a href={"/surge/surge"}>Surge</a> for graphics and visualisation.
                        </Text>
                    </Flex>
                </Flex>

                <Flex
                    direction={"row"}
                    align={"center"}
                    justify={"center"}
                    wrap={"wrap"}
                    css={{
                        gap: "$10",
                        pt: "$10",
                        pb: "$10",
                        px: "$10",
                        "@media (max-width: 1280px)": {
                            flexDirection: "column",
                            alignItems: "center",
                            justifyContent: "center",
                        }
                    }}
                    >
                    {
                        mlTutorials.map((tutorial) => {
                            return (
                                <ArticleCard
                                    key={tutorial.key}
                                    title={tutorial.title}
                                    textColour={tutorial.textColor}
                                    description={tutorial.description}
                                    // icon={tutorial.icon}
                                    image={tutorial.image}
                                    width={tutorial.width}
                                    height={tutorial.height}
                                    link={tutorial.link}
                                    tags={tutorial.tags}
                                />
                            )
                        })
                    }
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
