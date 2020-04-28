module.exports = {
    content: ['./src/**/*.html', './src/**/*.ts', './src/**/*.js', './src/**/*.vue'],
    defaultExtractor: content => content.match(/[\w-/:]+(?<!:)/g) || []
}