module.exports = function(eleventyConfig) {
    eleventyConfig.addPassthroughCopy("js");
    eleventyConfig.addPassthroughCopy("vms");
    return {
      dir: {
        input: "./",      // Equivalent to Jekyll's source property
        output: "./_site" // Equivalent to Jekyll's destination property
      }
    };
  };