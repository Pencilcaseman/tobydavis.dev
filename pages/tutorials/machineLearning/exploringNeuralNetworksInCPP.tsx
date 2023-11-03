import type {NextPage} from 'next';
import {Nav} from '../../../components/navbar/navbar';
import {Layout} from '../../../components/navbar/layout';
import {Box} from '../../../components/styles/box';
import {Footer} from '../../../components/footer';
import {Flex} from "../../../components/styles/flex";
import {Button, Divider, Text} from "@nextui-org/react";
import React from "react";
import {PageTag} from "../../../components/meta/pageTag";

const Home: NextPage = () => {
    return (
        <Layout>
            <PageTag pageTitle={"Toby Davis - Exploring Neural Networks in C++"}
                     contentTitle={"Exploring Neural Networks in C++"}
                     description={"A brief introduction to machine learning and neural networks, with C++ code examples."}
                     url={"https://tobydavis.dev/tutorials/machineLearning/exploringNeuralNetworksInCPP"}
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
                        maxWidth: "800px"
                    }}>
                        <Text h1 id={"highPerformance"}>Exploring Neural Networks in C++</Text>

                        <Text css={{
                            fontSize: "16pt"
                        }}>
                            One of the most difficult parts of machine learning is understanding the mathematics behind
                            it, and many tutorials and guides skip over this. Additionally, <em>implementing</em> the
                            algorithms can be difficult, and most tutorials simply use pre-existing libraries and
                            frameworks. In my opinion, this is not a good way to learn, as you do not develop the
                            fundamental understanding behind what your code is doing.

                            <br/><br/>

                            For these reasons, I decided to write (yet another) machine learning tutorial, but this time
                            I will be focusing on the mathematics and implementation of the algorithms, rather than
                            the use of frameworks. The code will be written in C++, using <a
                            href={"/librapid/librapid"}>LibRapid</a> for linear algebra and other mathematical
                            operations, and <a href={"/surge/surge"}>Surge</a> for graphics and visualisation.
                        </Text>

                        <Box css={{
                            padding: "$16",
                        }}>
                            <Button color={"success"} size={"lg"}>
                                <a href={"/files/exploringNeuralNetworksInCPP.pdf"}
                                   target={"_blank"}
                                   download>
                                    <Text size={"1.25rem"} color={"white"}>Download PDF</Text>
                                </a>
                            </Button>
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
