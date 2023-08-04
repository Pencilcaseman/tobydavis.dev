interface OpenGraphTagsProps {
    pageTitle?: string;
    contentTitle?: string;
    description?: string;
    image?: string;
    url?: string;
}

export const PageTag = (props: OpenGraphTagsProps) => {
    // Add open graph tags to the head of the page, as well as
    // twitter card tags.
    // Also set the favicon and iOS app icon

    return (
        <>
            <title>{props.pageTitle ?? props.contentTitle ?? "Toby Davis -- C++ Developer"}</title>
            <meta property="og:title" content={props.contentTitle ?? props.pageTitle ?? "Toby Davis -- C++ Developer"}/>
            <meta property="og:description" content={props.description}/>
            <meta property="og:image" content={props.image}/>
            <meta property="og:url" content={props.url}/>
            <meta property="og:type" content="website"/>
            <meta property="twitter:card" content="summary_large_image"/>
            <meta property="twitter:title"
                  content={props.contentTitle ?? props.pageTitle ?? "Toby Davis -- C++ Developer"}/>
            <meta property="twitter:description" content={props.description}/>
            <meta property="twitter:image" content={props.image}/>

            <link rel="icon" href="/public/favicon.ico"/>
            <link rel="apple-touch-icon" href="/public/logo_dark.png"/>
        </>
    )
}
