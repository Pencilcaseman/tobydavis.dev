import {icons} from "../../components/icons/icons";

export const Projects = () => ([
    {
        key: "librapid",
        title: "LibRapid",
        description: "A high-performance C++ library for mathematics and linear algebra.",
        icon: icons.flash,
        link: "/librapid/librapid"
    },
    {
        key: "symbomath",
        title: "SymboMath",
        description: "A fast and powerful symbolic mathematics library and research project.",
        icon: icons.rootX,
        link: "/symbomath/symbomath"
    },
    {
        key: "surge",
        title: "Surge",
        description: "A simple C++ graphics library for creative coding.",
        icon: icons.scale,
        link: "/surge/surge"
    },
    {
        key: "machineLearning",
        title: "Machine Learning",
        description: "Experiments with machine learning and artificial intelligence.",
        icon: icons.server,
        link: "/machineLearning/machineLearning"
    },
])
