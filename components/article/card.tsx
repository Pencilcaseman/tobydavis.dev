import React from "react";
import {Box} from "../styles/box";
import {Flex} from "../styles/flex";
import {Divider, Text, Card, Grid, Link, Col, Row, Button} from "@nextui-org/react";
import {Badge} from "../badge/badge";
import {useRouter} from "next/router";

function textShortener(text: string, width: number, height: number) {
    const maxChars = (width / 10) * (height / 10) / 10;
    if (text.length > maxChars) {
        return text.substring(0, maxChars - 3) + "...";
    } else {
        return text;
    }
}

function footerIfRequired(description: string, width: number | string, height: number | string) {
    if ((description?.length ?? 0) == 0) {
        return (
            <></>
        )
    }

    return (
        <Card.Footer
            isBlurred
            css={{
                position: "absolute",
                bgBlur: "#0f111466",
                borderTop: "$borderWeights$light solid $gray800",
                bottom: 0,
                zIndex: 1,
            }}
        >
            <Text css={{
                color: "#DDDDDD",
                fontSize: "14pt",
                fontWeight: "semibold",
            }}>
                {description}
            </Text>
        </Card.Footer>
    )
}

interface ArticleCardProps {
    title: string;
    textColour?: string;
    description?: string;
    icon?: React.ReactNode;
    image?: string;
    width?: number | string;
    height?: number | string;
    link?: string;
    tags?: string[];
}

export const ArticleCard = (props: ArticleCardProps = {
    title: "Title",
    description: "Description",
    icon: undefined,
    image: undefined,
    width: 500,
    height: 350,
    link: undefined,
    tags: [],
}) => {
    const router = useRouter();

    return (
        <Box>
            <Card isHoverable isPressable
                  onPress={() => {
                      if (props.link) {
                          if (props.link.startsWith("http")) {
                              window.open(props.link, "_blank")
                          } else {
                              router.push(props.link)
                                  .catch((err) => {
                                          console.log(err);
                                      }
                                  );
                          }
                      }
                  }}
                  css={{w: "100%", h: props.height ?? 400}}>
                <Card.Header css={{position: "absolute", zIndex: 1, top: 5}}>
                    <Col>
                        <Flex
                            direction={"row"}
                            align={"center"}
                            wrap={"wrap"}
                            css={{
                                gap: "$3",
                                pb: (props.tags?.length ?? 0) > 0 ? "$3" : "0",
                            }}
                        >
                            {
                                props.tags?.map((tag) => (
                                    <Badge key={"cardItem" + tag} text={tag}/>
                                ))
                            }
                        </Flex>
                        <Text h3 color={props.textColour ?? "white"}>
                            {props.title}
                        </Text>
                    </Col>
                </Card.Header>
                <Card.Body css={{p: 0}}>
                    <Card.Image
                        src={props.image ?? "https://fakeimg.pl/600x400"}
                        objectFit="cover"
                        width={props.width ?? "100%"}
                        height="100%"
                        alt="Card Image"
                    />
                </Card.Body>
                {footerIfRequired(props.description ?? "", props.width ?? 500, props.height ?? 350)}
            </Card>
        </Box>
    )
}
