import React from "react";

interface BadgeProps {
    color?: string;
    text?: string;
}

function getTextColor(bgColor: string) {
    if (!bgColor) return "black";

    // Convert hex color to RGB
    let color = bgColor.substring(1); // strip #
    let rgb = parseInt(color, 16);   // convert rrggbb to decimal
    let r = (rgb >> 16) & 0xff;  // extract red
    let g = (rgb >> 8) & 0xff;  // extract green
    let b = (rgb >> 0) & 0xff;  // extract blue

    // Calculate relative luminance (https://www.w3.org/TR/WCAG20/#relativeluminancedef)
    let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;

    // Normalize luminance
    luminance = luminance / 255;

    return luminance > 0.5 ? "black" : "white";
}

const badgeDefaults = [
    {
        text: ["librapid"],
        color: "#FF7F50"
    },
    {
        text: [
            "high performance computing",
            "hpc",
            "high-performance-computing",
            "parallel programming",
            "parallel-programming"
        ],
        color: "#DE3163"
    },
    {
        text: [
            "machine learning",
            "machine-learning",
            "artificial intelligence",
            "artificial-intelligence",
            "ml",
            "ai",
            "neural networks",
            "neural-networks"
        ],
        color: "#FFBF00"
    },
    {
        text: [
            "mathematics",
            "maths",
            "math",
            "linear algebra",
            "linear-algebra",
            "algebra",
            "calculus",
            "geometry",
            "statistics"
        ],
        color: "#DFFF00"
    },
    {
        text: [
            "c++",
            "cpp"
        ],
        color: "#9FE2BF"
    },
    {
        text: [
            "python",
            "py"
        ],
        color: "#40E0D0"
    },
    {
        text: [
            "java",
            "jvm"
        ],
        color: "#CCCCFF"
    },
    {
        text: [
            "javascript",
            "js",
            "node",
            "nodejs",
            "node.js",
            "react",
            "reactjs",
            "react.js",
            "next",
            "nextjs",
            "next.js",
            "typescript",
            "ts"
        ],
        color: "#6495ED"
    }
];

export const Badge = (props: BadgeProps) => {
    const defaultColor = "#1b9817";
    let color = props.color ?? defaultColor;

    if (color === defaultColor) {
        for (let badge of badgeDefaults) {
            if (badge.text.includes(props.text?.toLowerCase() ?? "")) {
                color = badge.color;
                break;
            }
        }
    }

    const style = {
        backgroundColor: color,
        padding: "1px 8px",
        borderRadius: "7px",
        display: "inline-block",
        fontSize: "12px",
        fontWeight: "bold",
        color: getTextColor(color ?? "#DA453E"),
    };

    return <span style={style}>{props.text}</span>;
};
