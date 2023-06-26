import React from 'react';

interface Props {
    fill?: string;
    size?: number;
    width?: number;
    height?: number;
}

const ChevronDownIcon = ({
                             fill,
                             size,
                             width = 24,
                             height = 24,
                             ...props
                         }: Props) => {
    return (
        <svg
            fill="none"
            height={size ?? height ?? 24}
            viewBox="0 0 24 24"
            width={size ?? width ?? 24}
            xmlns="http://www.w3.org/2000/svg"
            {...props}
        >
            <path
                d="m19.92 8.95-6.52 6.52c-.77.77-2.03.77-2.8 0L4.08 8.95"
                stroke={fill}
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeMiterlimit={10}
                strokeWidth={1.5}
            />
        </svg>
    );
};

const TagUserIcon = ({
                         fill,
                         size,
                         width = 24,
                         height = 24,
                         ...props
                     }: Props) => {
    return (
        <svg
            fill="none"
            height={size ?? height}
            viewBox="0 0 24 24"
            width={size ?? width}
            xmlns="http://www.w3.org/2000/svg"
            {...props}
        >
            <path
                d="M18 18.86h-.76c-.8 0-1.56.31-2.12.87l-1.71 1.69c-.78.77-2.05.77-2.83 0l-1.71-1.69c-.56-.56-1.33-.87-2.12-.87H6c-1.66 0-3-1.33-3-2.97V4.98c0-1.64 1.34-2.97 3-2.97h12c1.66 0 3 1.33 3 2.97v10.91c0 1.63-1.34 2.97-3 2.97Z"
                stroke={fill}
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeMiterlimit={10}
                strokeWidth={1.5}
            />
            <path
                d="M12 10a2.33 2.33 0 1 0 0-4.66A2.33 2.33 0 0 0 12 10ZM16 15.66c0-1.8-1.79-3.26-4-3.26s-4 1.46-4 3.26"
                stroke={fill}
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1.5}
            />
        </svg>
    );
};

const ServerIcon = ({fill, size, width = 24, height = 24, ...props}: Props) => {
    return (
        <svg
            fill="none"
            height={size ?? height}
            viewBox="0 0 24 24"
            width={size ?? width}
            xmlns="http://www.w3.org/2000/svg"
            {...props}
        >
            <path
                d="M19.32 10H4.69c-1.48 0-2.68-1.21-2.68-2.68V4.69c0-1.48 1.21-2.68 2.68-2.68h14.63C20.8 2.01 22 3.22 22 4.69v2.63C22 8.79 20.79 10 19.32 10ZM19.32 22H4.69c-1.48 0-2.68-1.21-2.68-2.68v-2.63c0-1.48 1.21-2.68 2.68-2.68h14.63c1.48 0 2.68 1.21 2.68 2.68v2.63c0 1.47-1.21 2.68-2.68 2.68ZM6 5v2M10 5v2M6 17v2M10 17v2M14 6h4M14 18h4"
                stroke={fill}
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1.5}
            />
        </svg>
    );
};

const FlashIcon = ({fill, size, width = 24, height = 24, ...props}: Props) => {
    return (
        <svg
            fill="none"
            height={size ?? height}
            viewBox="0 0 24 24"
            width={size ?? width}
            xmlns="http://www.w3.org/2000/svg"
            {...props}
        >
            <path
                d="M6.09 13.28h3.09v7.2c0 1.68.91 2.02 2.02.76l7.57-8.6c.93-1.05.54-1.92-.87-1.92h-3.09v-7.2c0-1.68-.91-2.02-2.02-.76l-7.57 8.6c-.92 1.06-.53 1.92.87 1.92Z"
                stroke={fill}
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeMiterlimit={10}
                strokeWidth={1.5}
            />
        </svg>
    );
};

const ActivityIcon = ({
                          fill,
                          size,
                          width = 24,
                          height = 24,
                          ...props
                      }: Props) => {
    return (
        <svg
            data-name="Iconly/Curved/Activity"
            height={size ?? height ?? 24}
            viewBox="0 0 24 24"
            width={size ?? width ?? 24}
            xmlns="http://www.w3.org/2000/svg"
            {...props}
        >
            <g
                fill="none"
                stroke={fill}
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeMiterlimit={10}
                strokeWidth={1.5}
            >
                <path d="M6.918 14.854l2.993-3.889 3.414 2.68 2.929-3.78"/>
                <path d="M19.668 2.35a1.922 1.922 0 11-1.922 1.922 1.921 1.921 0 011.922-1.922z"/>
                <path
                    d="M20.756 9.269a20.809 20.809 0 01.194 3.034c0 6.938-2.312 9.25-9.25 9.25s-9.25-2.312-9.25-9.25 2.313-9.25 9.25-9.25a20.931 20.931 0 012.983.187"/>
            </g>
        </svg>
    );
};

const ScaleIcon = ({fill, size, width = 24, height = 24, ...props}: Props) => {
    return (
        <svg
            fill="none"
            height={size ?? height}
            viewBox="0 0 24 24"
            width={size ?? width}
            xmlns="http://www.w3.org/2000/svg"
            {...props}
        >
            <path
                d="M9 22h6c5 0 7-2 7-7V9c0-5-2-7-7-7H9C4 2 2 4 2 9v6c0 5 2 7 7 7ZM18 6 6 18"
                stroke={fill}
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1.5}
            />
            <path
                d="M18 10V6h-4M6 14v4h4"
                stroke={fill}
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1.5}
            />
        </svg>
    );
};

const RootXIcon = ({fill, size, width = 24, height = 24, ...props}: Props) => {
    return (
        <svg
            width={size ?? width}
            height={size ?? height}
            viewBox="0 0 22 18"
            fill="none"
            xmlns="http://www.w3.org/2000/svg">
            <path
                d="M1 9.92855L4.45017 16.974M4.45248 17L8.378 3.18027C8.74446 1.89016 9.92269 1 11.2638 1H21M11.184 16.9734L14.58 10.6408L17.9759 4.30829M11.2157 4.3083L18.0076 16.9734"
                stroke="#F17777" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
    )
}

const DevToIcon = ({fill, size, width = 24, height = 24, ...props}: Props) => {
    return (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            aria-label="dev.to"
            role="img"
            viewBox="0 0 512 512"
            width={size ?? width}
            height={size ?? height}
            fill="#4497c1"
            stroke="#4497c1">

            <g id="SVGRepo_bgCarrier" stroke-width="0"/>

            <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"/>

            <g id="SVGRepo_iconCarrier">

                <rect width="512" height="512" rx="15%"/>

                <path fill="#ffffff"
                      d="M140.47 203.94h-17.44v104.47h17.45c10.155-.545 17.358-8.669 17.47-17.41v-69.65c-.696-10.364-7.796-17.272-17.48-17.41zm45.73 87.25c0 18.81-11.61 47.31-48.36 47.25h-46.4V172.98h47.38c35.44 0 47.36 28.46 47.37 47.28zm100.68-88.66H233.6v38.42h32.57v29.57H233.6v38.41h53.29v29.57h-62.18c-11.16.29-20.44-8.53-20.72-19.69V193.7c-.27-11.15 8.56-20.41 19.71-20.69h63.19zm103.64 115.29c-13.2 30.75-36.85 24.63-47.44 0l-38.53-144.8h32.57l29.71 113.72 29.57-113.72h32.58z"/>

            </g>

        </svg>
    )
}

const LinkedInIcon = ({fill, size, width = 24, height = 24, ...props}: Props) => {
    return (
        <svg
            fill="#7c3bde"
            height={size ?? height}
            width={size ?? width}
            version="1.1"
            id="Layer_1"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="-143 145 512 512"
            stroke="#7c3bde">

            <g id="SVGRepo_bgCarrier" stroke-width="0"/>

            <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"/>

            <g id="SVGRepo_iconCarrier">
                <path
                    d="M329,145h-432c-22.1,0-40,17.9-40,40v432c0,22.1,17.9,40,40,40h432c22.1,0,40-17.9,40-40V185C369,162.9,351.1,145,329,145z M41.4,508.1H-8.5V348.4h49.9V508.1z M15.1,328.4h-0.4c-18.1,0-29.8-12.2-29.8-27.7c0-15.8,12.1-27.7,30.5-27.7 c18.4,0,29.7,11.9,30.1,27.7C45.6,316.1,33.9,328.4,15.1,328.4z M241,508.1h-56.6v-82.6c0-21.6-8.8-36.4-28.3-36.4 c-14.9,0-23.2,10-27,19.6c-1.4,3.4-1.2,8.2-1.2,13.1v86.3H71.8c0,0,0.7-146.4,0-159.7h56.1v25.1c3.3-11,21.2-26.6,49.8-26.6 c35.5,0,63.3,23,63.3,72.4V508.1z"/>
            </g>
        </svg>
    )
}

const GitHubIcon = ({fill, size, width = 24, height = 24, ...props}: Props) => {
    return (
        <svg
            width={size ?? width}
            height={size ?? height}
            viewBox="0 0 73 73"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            fill="#3bcea2"
            stroke="#3bcea2">

            <g id="SVGRepo_bgCarrier" stroke-width="0"/>

            <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"/>

            <g id="SVGRepo_iconCarrier"><title>team-collaboration/version-control/github</title>
                <desc>Created with Sketch.</desc>
                <defs></defs>
                <g id="team-collaboration/version-control/github" stroke="none" stroke-width="1" fill="none"
                   fill-rule="evenodd">
                    <g id="container" transform="translate(2.000000, 2.000000)" fill-rule="nonzero">
                        <rect id="mask" stroke="#43ad5d" stroke-width="2" fill="#43ad5d" x="-1" y="-1" width="71"
                              height="71" rx="14"></rect>
                        <path
                            d="M58.3067362,21.4281798 C55.895743,17.2972267 52.6253846,14.0267453 48.4948004,11.615998 C44.3636013,9.20512774 39.8535636,8 34.9614901,8 C30.0700314,8 25.5585181,9.20549662 21.4281798,11.615998 C17.2972267,14.0266224 14.0269912,17.2972267 11.615998,21.4281798 C9.20537366,25.5590099 8,30.0699084 8,34.9607523 C8,40.8357654 9.71405782,46.1187277 13.1430342,50.8109917 C16.5716416,55.5036246 21.0008949,58.7507436 26.4304251,60.5527176 C27.0624378,60.6700211 27.5302994,60.5875152 27.8345016,60.3072901 C28.1388268,60.0266961 28.290805,59.6752774 28.290805,59.2545094 C28.290805,59.1842994 28.2847799,58.5526556 28.2730988,57.3588401 C28.2610487,56.1650247 28.2553926,55.1235563 28.2553926,54.2349267 L27.4479164,54.3746089 C26.9330843,54.468919 26.2836113,54.5088809 25.4994975,54.4975686 C24.7157525,54.4866252 23.9021284,54.4044881 23.0597317,54.2517722 C22.2169661,54.1004088 21.4330982,53.749359 20.7075131,53.1993604 C19.982297,52.6493618 19.4674649,51.9294329 19.1631397,51.0406804 L18.8120898,50.2328353 C18.5780976,49.6950097 18.2097104,49.0975487 17.7064365,48.4426655 C17.2031625,47.7871675 16.6942324,47.3427912 16.1794003,47.108799 L15.9336039,46.9328437 C15.7698216,46.815909 15.6178435,46.6748743 15.4773006,46.511215 C15.3368806,46.3475556 15.2317501,46.1837734 15.1615401,46.0197452 C15.0912072,45.855594 15.1494901,45.7209532 15.3370036,45.6153308 C15.5245171,45.5097084 15.8633939,45.4584343 16.3551097,45.4584343 L17.0569635,45.5633189 C17.5250709,45.6571371 18.104088,45.9373622 18.7947525,46.4057156 C19.4850481,46.8737001 20.052507,47.4821045 20.4972521,48.230683 C21.0358155,49.1905062 21.6846737,49.9218703 22.4456711,50.4251443 C23.2060537,50.9284182 23.9727072,51.1796248 24.744894,51.1796248 C25.5170807,51.1796248 26.1840139,51.121096 26.7459396,51.0046532 C27.3072505,50.8875956 27.8338868,50.7116403 28.3256025,50.477771 C28.5362325,48.9090515 29.1097164,47.7039238 30.0455624,46.8615271 C28.7116959,46.721353 27.5124702,46.5102313 26.4472706,46.2295144 C25.3826858,45.9484285 24.2825656,45.4922482 23.1476478,44.8597436 C22.0121153,44.2280998 21.0701212,43.44374 20.3214198,42.5080169 C19.5725954,41.571802 18.9580429,40.3426971 18.4786232,38.821809 C17.9989575,37.300306 17.7590632,35.5451796 17.7590632,33.5559381 C17.7590632,30.7235621 18.6837199,28.3133066 20.5326645,26.3238191 C19.6665366,24.1944035 19.7483048,21.8072644 20.778215,19.1626478 C21.4569523,18.951772 22.4635002,19.1100211 23.7973667,19.6364115 C25.1314792,20.1630477 26.1082708,20.6141868 26.7287253,20.9882301 C27.3491798,21.3621504 27.8463057,21.6790175 28.2208409,21.9360032 C30.3978419,21.3277217 32.644438,21.0235195 34.9612442,21.0235195 C37.2780503,21.0235195 39.5251383,21.3277217 41.7022622,21.9360032 L43.0362517,21.0938524 C43.9484895,20.5319267 45.0257392,20.0169716 46.2654186,19.5488642 C47.5058357,19.0810026 48.4543466,18.9521409 49.1099676,19.1630167 C50.1627483,21.8077563 50.2565666,24.1947724 49.3901927,26.324188 C51.2390143,28.3136755 52.1640399,30.7245457 52.1640399,33.556307 C52.1640399,35.5455485 51.9232849,37.3062081 51.444357,38.8393922 C50.9648143,40.3728223 50.3449746,41.6006975 49.5845919,42.5256002 C48.8233486,43.4503799 47.8753296,44.2285916 46.7404118,44.8601125 C45.6052481,45.4921252 44.504759,45.9483056 43.4401742,46.2293914 C42.3750975,46.5104772 41.1758719,46.7217219 39.8420054,46.8621419 C41.0585683,47.9149226 41.6669728,49.5767225 41.6669728,51.846804 L41.6669728,59.2535257 C41.6669728,59.6742937 41.8132948,60.0255895 42.1061847,60.3063064 C42.3987058,60.5865315 42.8606653,60.6690374 43.492678,60.5516109 C48.922946,58.7498829 53.3521992,55.5026409 56.7806837,50.810008 C60.2087994,46.117744 61.923472,40.8347817 61.923472,34.9597686 C61.9222424,30.0695396 60.7162539,25.5590099 58.3067362,21.4281798 Z"
                            id="Shape" fill="#FFFFFF"></path>
                    </g>
                </g>
            </g>

        </svg>
    )
}

const DiscordIcon = ({fill, size, width = 24, height = 24, ...props}: Props) => {
    return (
        <svg
            width={size ?? width}
            height={size ?? height}
            viewBox="0 -28.5 256 256"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            preserveAspectRatio="xMidYMid">
            <g>
                <path
                    d="M216.856339,16.5966031 C200.285002,8.84328665 182.566144,3.2084988 164.041564,0 C161.766523,4.11318106 159.108624,9.64549908 157.276099,14.0464379 C137.583995,11.0849896 118.072967,11.0849896 98.7430163,14.0464379 C96.9108417,9.64549908 94.1925838,4.11318106 91.8971895,0 C73.3526068,3.2084988 55.6133949,8.86399117 39.0420583,16.6376612 C5.61752293,67.146514 -3.4433191,116.400813 1.08711069,164.955721 C23.2560196,181.510915 44.7403634,191.567697 65.8621325,198.148576 C71.0772151,190.971126 75.7283628,183.341335 79.7352139,175.300261 C72.104019,172.400575 64.7949724,168.822202 57.8887866,164.667963 C59.7209612,163.310589 61.5131304,161.891452 63.2445898,160.431257 C105.36741,180.133187 151.134928,180.133187 192.754523,160.431257 C194.506336,161.891452 196.298154,163.310589 198.110326,164.667963 C191.183787,168.842556 183.854737,172.420929 176.223542,175.320965 C180.230393,183.341335 184.861538,190.991831 190.096624,198.16893 C211.238746,191.588051 232.743023,181.531619 254.911949,164.955721 C260.227747,108.668201 245.831087,59.8662432 216.856339,16.5966031 Z M85.4738752,135.09489 C72.8290281,135.09489 62.4592217,123.290155 62.4592217,108.914901 C62.4592217,94.5396472 72.607595,82.7145587 85.4738752,82.7145587 C98.3405064,82.7145587 108.709962,94.5189427 108.488529,108.914901 C108.508531,123.290155 98.3405064,135.09489 85.4738752,135.09489 Z M170.525237,135.09489 C157.88039,135.09489 147.510584,123.290155 147.510584,108.914901 C147.510584,94.5396472 157.658606,82.7145587 170.525237,82.7145587 C183.391518,82.7145587 193.761324,94.5189427 193.539891,108.914901 C193.539891,123.290155 183.391518,135.09489 170.525237,135.09489 Z"
                    fill="#5865F2" fill-rule="nonzero">

                </path>
            </g>
        </svg>
    )
}

const StackOverflowIcon = ({fill, size, width = 24, height = 24, ...props}: Props) => {
    return (
        <svg
            width={size ?? width}
            height={size ?? height}
            viewBox="0 0 32 32"
            fill="none"
            xmlns="http://www.w3.org/2000/svg">
            <path
                d="M16 2C8.27812 2 2 8.27812 2 16C2 23.7219 8.27812 30 16 30C23.7219 30 30 23.7219 30 16C30 8.27812 23.7219 2 16 2Z"
                fill="#E08919"/>
            <path d="M19.5405 18.3626L12.1082 17.2883L12.3658 15.6605L19.6337 17.0581L19.5405 18.3626Z" fill="white"/>
            <path d="M19.672 16.6306L13.0948 13.5612L13.6977 12.2458L20.2749 15.3152L19.672 16.6306Z" fill="white"/>
            <path d="M20.549 14.9863L14.9583 10.3274L15.8901 9.23122L21.4808 13.8901L20.549 14.9863Z" fill="white"/>
            <path d="M17.37 7.80615L18.521 6.9292L22.851 12.7391L21.7 13.6161L17.37 7.80615Z" fill="white"/>
            <path d="M19.5624 20.5221H11.889V18.8778H19.5624V20.5221Z" fill="white"/>
            <path d="M20.6586 17.2334V21.6182H10.7927V17.2334H9.14844V23.2625H22.3029V17.2334H20.6586Z" fill="white"/>
        </svg>
    )
}

const MidjourneyIcon = ({fill, size, width = 24, height = 24, ...props}: Props) => {
    return (
        <svg
            width={size ?? width}
            height={size ?? height}
            id="Layer_2"
            data-name="Layer 2"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 505.85 421.37">
            <path className="cls-1" fill={fill ?? "#f36c21"}
                  d="m360.39,401.81c10.08,6.46,19.55,6.88,29.33,2.07,4.63-2.28,8.56-5.6,12.57-8.8,4.17-3.32,8.22-6.83,12.63-9.79,16.78-11.23,34.79-10.75,50.89,1.38,5.07,3.82,9.73,8.18,14.6,12.26,5.42,4.54,11.16,8.22,18.59,8.67,4.16.25,7.09,2.8,6.84,7.18-.27,4.66-3.44,6.73-8.02,6.58-8.77-.29-16.3-3.87-23.02-9.18-5.3-4.19-10.26-8.82-15.51-13.08-12.69-10.27-25.06-10.71-38.28-1.17-5.98,4.31-11.46,9.32-17.44,13.63-17.85,12.85-36.28,12.73-54.06-.3-5.28-3.87-10.15-8.31-15.34-12.32-13.51-10.42-25.82-10.45-39.27-.08-5.19,4-10.07,8.42-15.35,12.3-16.89,12.42-34.78,12.67-51.89.61-5.85-4.12-11.19-8.98-16.87-13.36-12.11-9.35-21.69-9.71-34.32-1.05-6.25,4.28-12.07,9.19-18.31,13.5-17.59,12.15-35.54,12-53.68,1.16-8.31-4.97-15.5-11.42-23.1-17.34-11.57-9.01-13.73-9.29-26.21-1.49-9.21,5.76-18.02,12.14-27.09,18.12-5.49,3.62-11.22,6.9-17.82,8.01-4.59.78-9.09.19-10.09-5.33-.97-5.39,2.38-7.64,7.43-8.33,4.77-.66,8.82-3.29,12.75-5.88,8.38-5.53,16.59-11.34,25.03-16.77,2.88-1.85,3.26-3.39,1.54-6.35-3.4-5.84-6.4-11.91-9.52-17.91-4.81-9.26-2.62-13.18,7.93-13.89,20.05-1.34,40.11-2.54,60.16-3.84,68.54-4.43,137.07-8.89,205.61-13.32,51.56-3.33,103.12-6.62,154.68-9.94,4.25-.27,8.44-.42,10.27,4.52,1.87,5.05-1.85,7.48-5.04,10.06-32.46,26.34-67.56,48.5-105.25,66.57-1.45.7-2.84,1.53-5.35,2.89Zm84.82-66.96c-.24-.55-.47-1.1-.71-1.65-11.23.66-22.47,1.26-33.69,1.99-67.09,4.33-134.18,8.67-201.27,13.03-48.27,3.13-96.54,6.24-144.81,9.48-11.65.78-11.69,1.03-6.18,11.31,1.16,2.16,1.5,5.73,5.17,5.08,10.68-1.89,18.47,3.7,26.2,9.66,6.49,5,12.77,10.32,19.56,14.88,15.84,10.65,28.09,10.2,43.6-1.06,5.64-4.1,10.99-8.6,16.8-12.43,16.7-11.02,32.48-10.45,48.4,1.56,5.72,4.32,11.05,9.16,16.85,13.39,12.14,8.85,24.22,8.78,36.31-.14,4.94-3.65,9.48-7.84,14.35-11.59,19.11-14.76,37.63-14.99,56.79-.27,3.89,2.98,6.96,3.07,11.07,1.16,22.9-10.63,44.97-22.76,66.11-36.54,8.68-5.66,16.99-11.88,25.47-17.85Z"/>
            <path className="cls-1" fill={fill ?? "#f36c21"}
                  d="m77.79,0c1.03.4,2.97,1.03,4.8,1.89,78.08,36.71,141.52,90.46,188.85,162.82,23.72,36.27,41.99,75.02,52.26,117.3,1.54,6.36,2.72,12.82,3.74,19.29,1.25,7.93-3.5,11.65-11,8.68-35.38-14-71.97-22.31-110.11-22.73-45.32-.5-86.74,12.96-125.33,36.04-2.47,1.47-4.77,3.3-7.94,2.43-5.16-1.42-6.83-7.43-2.89-11.82,25.79-28.78,38.25-63.49,45.08-100.74,9.15-49.88,2.02-97.85-16.78-144.52-7.82-19.41-16.58-38.34-26.81-56.6C68.1,5.69,70.66-.09,77.79,0Zm17.09,23.15c-.91,2.86.57,4.27,1.32,5.82,8.51,17.52,15.97,35.48,22.13,53.96,24.09,72.29,19.01,142.34-16.12,210.09-.51.98-1.68,1.94-.25,3.41,33.02-14.85,67.54-23.49,104-23.11,36.1.38,70.97,7.74,106.24,20.02-13.75-60.2-41.81-111.68-79.49-158.22-37.97-46.91-84.43-83.48-137.83-111.96Z"/>
            <path className="cls-1" fill={fill ?? "#f36c21"}
                  d="m212.66,51.53c9.09,1.04,17.91,4.91,26.43,9.45,26.69,14.21,48.9,34.2,70.99,54.5,54.66,50.22,89.95,112.6,115.33,181.37,1.31,3.55,2.62,7.17-.47,10.35-3.37,3.47-7.12,1.83-10.83.37-11.59-4.57-22.85-10.45-35.84-9.74-4.69.26-9.38.89-14.02,1.66-8.3,1.39-10.15.29-12.44-7.59-15.2-52.24-36.83-101.62-65.93-147.66-20.02-31.67-45.97-57.54-76.75-78.68-2.61-1.79-5.12-3.5-5.28-7.03-.21-4.32,3.09-7.14,8.82-6.99Zm194.15,237.4c-14.18-37.37-31.65-72.48-54.42-104.78-19.12-27.12-70.54-79.57-81.57-83.52,5.89,7.54,12.18,14.85,17.66,22.74,32.86,47.35,56.24,99.29,73.43,154.1,1.92,6.14,4.08,8.04,10.42,7,11.71-1.92,23.1.6,34.49,4.46Z"/>
        </svg>
    )
}

export const icons = {
    chevron: <ChevronDownIcon fill="currentColor" size={16}/>,
    scale: <ScaleIcon fill="var(--nextui-colors-warning)" size={30}/>,
    activity: <ActivityIcon fill="var(--nextui-colors-secondary)" size={30}/>,
    flash: <FlashIcon fill="var(--nextui-colors-primary)" size={30}/>,
    server: <ServerIcon fill="var(--nextui-colors-success)" size={30}/>,
    user: <TagUserIcon fill="var(--nextui-colors-error)" size={30}/>,
    rootX: <RootXIcon fill="var(--nextui-colors-error)" size={30}/>,
    devTo: <DevToIcon fill="var(--nextui-colors-error)" size={30}/>,
    linkedIn: <LinkedInIcon fill="var(--nextui-colors-error)" size={30}/>,
    github: <GitHubIcon fill="var(--nextui-colors-error)" size={30}/>,
    discord: <DiscordIcon fill="var(--nextui-colors-error)" size={30}/>,
    stackOverflow: <StackOverflowIcon fill="var(--nextui-colors-error)" size={30}/>,
    midjourney: <MidjourneyIcon fill="var(--nextui-colors-error)" size={30}/>,
};