module.exports = {
  plugins: {
    'postcss-import': {

    },
    'tailwindcss': {
      theme: {
        fontFamily: {
          'display': ['Oswald'],
          'body': ['Roboto Condensed'],
        },
        extend: {
          colors: {
            'mint-leaf': '#00b894',
            'light-greenish-blue': '#55efc4',
            'secondary': '#1e272e',
            'primary': '#3498db'
          }
        }
      },
    },
    'autoprefixer': {},
    // 'postcss-typography': {
    //   includeNormalize: false,
    // },
    'postcss-font-magician': {
      variants: {
        'Roboto Condensed': {
          '300': [],
          '400': [],
          '700': []
        },
        'Oswald': {
          '300': [],
          '400': [],
          '700': []
        }
      },
      foundries: ['google']
    },
  }
}