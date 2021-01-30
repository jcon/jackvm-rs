module.exports = {
  // Basic PurgeCSS config.  Read more at
  // https://tailwindcss.com/docs/controlling-file-size/
  purge: [
    "./src/site/**/*.njk",
    "./src/js/**/*.js"
  ],
  theme: {
    extend: {
      width: {
        '256px': '256px',
        '512px': '512px',
        '1024px': '1024px',
      },
      height: {
        '256px': '256px',
        '512px': '512px',
        '1024px': '1024px',
      }
    }
  },
  variants: {},
  plugins: [],
};
