module.exports = {
  plugins: [
    require('autoprefixer'),
    require('cssnano', {
      discardComments: true
    })
  ]
}