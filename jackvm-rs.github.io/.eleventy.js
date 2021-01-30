const fs = require("fs");
const path = require("path");

const manifestPath = path.resolve(__dirname, "_site", "assets", "manifest.json");
const manifest = JSON.parse(
  fs.readFileSync(manifestPath, { encoding: "utf8" })
);

module.exports = function(eleventyConfig) {
  // Layout aliases make templates more portable.
  eleventyConfig.addLayoutAlias("default", "layouts/default.njk");
  eleventyConfig.addLayoutAlias("player", "layouts/player.njk");

  // Adds a universal shortcode to return the URL to a webpack asset. In Nunjack templates:
  // {% webpackAsset 'main.js' %} or {% webpackAsset 'main.css' %}
  eleventyConfig.addShortcode("webpackAsset", function(name) {
    if (!manifest[name]) {
      throw new Error(`The asset ${name} does not exist in ${manifestPath}`);
    }
    // return '/jackvm-rs' + manifest[name];
    let prefix = '';
    if (process.env.NODE_ENV === 'production') {
      prefix = '/jackvm-rs';
    }
    return prefix + manifest[name];
  });

  // Copy all images directly to _site.
  eleventyConfig.addPassthroughCopy({ "src/img": "img" });

  // Copy all vms directly to _site.
  eleventyConfig.addPassthroughCopy({ "src/vms": "vms" });

  // Copy external dependencies to _site.
  eleventyConfig.addPassthroughCopy({ "src/vendor": "vendor" });

  // Reload the page every time the JS/CSS are changed.
  eleventyConfig.setBrowserSyncConfig({ files: [manifestPath] });

  // A debug utility.
  eleventyConfig.addFilter("dump", obj => {
    return util.inspect(obj);
  });

  return {
    dir: {
      input: "src/site",
      includes: "_includes", // relative to dir.input
      output: "_site",
    },
    htmlTemplateEngine: "njk",
    markdownTemplateEngine: "njk",
    passthroughFileCopy: true,
  };
};
