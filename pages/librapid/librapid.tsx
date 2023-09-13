import type {NextPage} from 'next';
import {Nav} from '../../components/navbar/navbar';
import {Layout} from '../../components/navbar/layout';
import {Trusted} from '../../components/trusted';
import {Box} from '../../components/styles/box';
import {Features1} from '../../components/features1';
import {Features2} from '../../components/features2';
import {Features3} from '../../components/features3';
import {Testimonials} from '../../components/tesminonials';
import {Statistics} from '../../components/statistics';
import {Plans} from '../../components/plans';
import {Faq} from '../../components/faq';
import {Trial} from '../../components/trial';
import {Footer} from '../../components/footer';
import {Flex} from "../../components/styles/flex";
import {Button, Divider, Image, Input, Link, Text, useTheme} from "@nextui-org/react";
import {CheckIcon} from "../../components/icons/CheckIcon";
import React from "react";
import {PageTag} from "../../components/meta/pageTag";
import {ArticleCard} from "../../components/article/card";
import {useTheme as useNextTheme} from "next-themes";

const homeImageDark = "https://raw.githubusercontent.com/LibRapid/librapid_extras/master/branding/LibRapid_dark.png";
const homeImageLight = "https://raw.githubusercontent.com/LibRapid/librapid_extras/master/branding/LibRapid_light.png";

const Home: NextPage = () => {
    const {setTheme} = useNextTheme();
    const {isDark, type} = useTheme();

    return (
        <Layout>
            <PageTag pageTitle={"Toby Davis - LibRapid"}
                     contentTitle={"LibRapid for C++"}
                     description={"LibRapid is a high-performance C++ library for machine learning, numerical computing and more."}
                     url={"https://tobydavis.dev/librapid/librapid"}
                     image={homeImageDark}
            />

            <Nav/>

            <Box as="main" css={{
                height: "100%",
                overflow: "auto"
            }}>
                <Flex css={{
                    flexDirection: "row",
                    justifyContent: "center",
                    height: "80vh",
                }}>
                    <Flex
                        css={{
                            "gap": "$10",
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
                                '& img': {
                                    width: "850px",
                                    objectFit: "contain",
                                }
                            }}
                        >
                            <Link href={"https://github.com/LibRapid/librapid"} target={"_blank"}>
                                <Image
                                    src={isDark ? homeImageDark : homeImageLight}
                                    alt="LibRapid Logo"

                                />
                            </Link>
                        </Box>

                        <Flex direction={"column"}
                              align={"center"}
                              justify={"center"}
                              css={{
                                  pt: "$8",
                                  minWidth: "550px",

                                  "@media (max-width: 768px)": {
                                      flexWrap: "wrap",
                                      minWidth: "0px",
                                  },
                              }}>
                            <Link href={"#highPerformance"}>
                                <Text h1 css={{
                                    display: "inline",
                                    textGradient: "45deg, $blue600 -20%, $pink600 75%",
                                    fontWeight: "$bold"
                                }}>
                                    High-Performance C++
                                </Text>
                            </Link>

                            <Link href={"#modernAPI"}>
                                <Text h1 css={{
                                    display: "inline",
                                    textGradient: "120deg, $red600 -20%, $yellow600 80%",
                                    fontWeight: "$bold"
                                }}>
                                    Intuitive, Modern API
                                </Text>
                            </Link>

                            <Link href={"#crossPlatform"}>
                                <Text h1 css={{
                                    display: "inline",
                                    textGradient: "45deg, $green600 -20%, $blue600 100%",
                                    fontWeight: "$bold"
                                }}>
                                    Cross-Platform
                                </Text>
                            </Link>

                            <Link href={"#opensource"}>
                                <Text h1 css={{
                                    display: "inline",
                                    textGradient: "120deg, $pink600 -20%, $red600 100%",
                                    fontWeight: "$bold"
                                }}>
                                    Open-Source
                                </Text>
                            </Link>
                        </Flex>
                    </Flex>
                </Flex>

                <Flex css={{
                    flexDirection: "row",
                    alignItems: "center",
                    justifyContent: "center",
                    pt: "$20",
                }}>
                    <Flex css={{
                        flexDirection: "column",
                        alignItems: "center",
                        justifyContent: "center",
                        pt: "$20",
                        maxWidth: "75%",
                    }}>
                        <Text h1 id={"highPerformance"}>High-Performance C++</Text>

                        <Text css={{
                            fontSize: "18pt",
                        }}>
                            LibRapid is a high-performance C++ library for machine learning, numerical computing and
                            more. Every aspect of LibRapid is designed with performance in mind, from the low-level
                            algorithms to the high-level API.

                            <br/><br/>

                            LibRapid also supports OpenCL and CUDA backends, allowing you to run your code on GPUs,
                            FPGAs and other accelerators. Changing the backend you want to use is as simple as adding a
                            second template argument to the Array class. For
                            example, <code>{"librapid::Array<float, librapid::Backend::CUDA>"}</code> will create an
                            Array on the GPU, accelerated by CUDA. LibRapid can automatically detect OpenCL and CUDA on
                            your system, so there is no need to spend hours configuring your build system.

                            <br/><br/>

                            Making efficient use of SIMD vectorization, multithreading and cache locality, LibRapid
                            is capable of performing faster than many other popular libraries, including <Link
                            href={"https://eigen.tuxfamily.org/"}>Eigen</Link> and <Link
                            href={"https://github.com/xtensor-stack/xtensor"}>XTensor</Link>.

                            <br/><br/>

                            LibRapid also integrates nicely with BLAS libraries, such as <Link
                            href={"https://github.com/xianyi/OpenBLAS"}>OpenBLAS</Link> and <Link
                            href={"https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html"}>Intel&apos;s
                            Math Kernel Library</Link>, for even greater performance. Additionally, these libraries are
                            automatically detected by LibRapid&apos;s CMake build system, so you don&apos;t have to
                            worry about configuring them yourself. LibRapid can even download pre-built OpenBLAS
                            binaries for you, so you can get started right away without installing anything.
                        </Text>

                        <Text h1 id={"modernAPI"} css={{
                            pt: "$20",
                        }}>Intuitive, Modern API</Text>

                        <Flex css={{
                            flexDirection: "column",
                            alignItems: "center",
                            justifyContent: "center",
                            gap: "$8",

                            "@md": {
                                flexDirection: "row",
                            }
                        }}>
                            <Box>
                                <Text css={{
                                    fontSize: "18pt",
                                    minWidth: "400px",

                                    "@md": {
                                        minWidth: "0px",
                                    }
                                }}>
                                    LibRapid&apos;s API is designed to be as intuitive as possible, providing users with
                                    a familiar experience. LibRapid uses expressive classes and functions, operator
                                    overloading and compile-time function selection to ensure that your code is as
                                    concise and readable as it can be.

                                    <br/><br/>

                                    LibRapid also folds combined operations into a single function call, reducing the
                                    number of temporary objects created and improving performance. Even BLAS operations
                                    are often folded into a single function call, eliminating the need for complex
                                    functions like <code>gemm</code> and <code>gemv</code>.

                                    <br/><br/>

                                    Functions in LibRapid also work on a wide variety of types, including floating point
                                    values, complex numbers, arrays, matrices, vectors and more. This allows you to
                                    write generic code that works on any type of data without sacrificing performance.
                                </Text>
                            </Box>

                            <Box>
                                <Image src={"/images/simpleLibRapidExample.png"} alt={"LibRapid Example"} css={{
                                    minWidth: "600px",
                                    maxWidth: "800px"
                                }}/>
                            </Box>
                        </Flex>

                        <Text h1 id={"crossPlatform"} css={{
                            pt: "$20",
                        }}>Cross-Platform</Text>

                        <Text css={{
                            fontSize: "18pt",
                        }}>
                            LibRapid is designed to be cross-platform, and is tested on Windows, Linux and MacOS with
                            GCC, Clang and MSVC. Every time code is pushed to the repository, LibRapid is compiled,
                            tested and benchmarked on all of these platforms and compilers, ensuring that LibRapid is
                            always working as expected.
                        </Text>

                        <Text h1 id={"openSource"} css={{
                            pt: "$20",
                        }}>Open Source</Text>

                        <Text css={{
                            fontSize: "18pt",
                        }}>
                            LibRapid is completely open-source, and is licensed under the <Link
                            href={"https://github.com/LibRapid/librapid/blob/master/LICENSE"} target={"_blank"}>MIT
                            License</Link>, so you
                            can use it in your personal and commercial projects without any restrictions.

                            <br/><br/>

                            Any contributions to LibRapid are greatly appreciated and allow me to continue improving the
                            library. Donations are also welcome (though code contributions are preferred), and can be
                            made via <Link href={"https://github.com/sponsors/Pencilcaseman"} target={"_blank"}>GitHub
                            Sponsors</Link> or <Link
                            href={"https://paypal.me/pencilcaseman?country.x=GB&locale.x=en_GB"}
                            target={"_blank"}>PayPal</Link>.
                        </Text>
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
