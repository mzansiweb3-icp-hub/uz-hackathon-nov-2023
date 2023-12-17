/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        black1: "#000",
        dimgray: {
          "100": "#585b65",
          "200": "#555",
        },
        "gray-white": "#fff",
        royalblue: "#3050ce",
        black: "#060606",
        gray: {
          "100": "#fdfdfd",
          "200": "#1a1a1a",
        },
        "content-main": "#02234d",
        darkgray: "#a8b2c0",
        whitesmoke: {
          "100": "#ededed",
          "200": "#eeeaea",
        },
      },
      spacing: {},
      fontFamily: {
        raleway: "Raleway",
        montserrat: "Montserrat",
        roboto: "Roboto",
        "montserrat-alternates": "'Montserrat Alternates'",
      },
      borderRadius: {
        "31xl": "50px",
      },
    },
    fontSize: {
      lg: "18px",
      base: "16px",
      "5xl": "24px",
      xl: "20px",
      "13xl": "32px",
      "35xl-1": "54.1px",
      smi: "13px",
      "11xl": "30px",
      sm: "14px",
      inherit: "inherit",
    },
    screens: {
      lg: {
        max: "1200px",
      },
      md: {
        max: "960px",
      },
    },
  },
  corePlugins: {
    preflight: false,
  },
};
