const buildingForProduction = process.env.BUILD_MODE === 'production';

export default {
  build: {
    manifest: true,
    rollupOptions: {
      input: {
        colorScheme: './js/color-scheme.js',
        main: './js/main.js',
      }
    },
    outDir: 'static/bundle',
    assetsDir: '',
    minify: buildingForProduction
  },

  publicDir: false,
  clearScreen: false
}
