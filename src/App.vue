<template>
  <router-view />
</template>
<style>
@font-face {
  font-family: 'CustomFont';
  src: local('HarmonyOS Sans Regular'),
       url('@/assets/HarmonyOS_Sans_Regular.ttf') format('truetype');
  font-weight: normal;
  font-style: normal;
  font-display: swap;
}

html, body {
  overflow: hidden;
  margin: 0;
  padding: 0;
  font-family: 'CustomFont', system-ui, -apple-system, sans-serif;
  color: #000000;
  font-size: 16px;
  line-height: 1.5;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-rendering: optimizeLegibility;
}

* {
  box-sizing: border-box;
}
</style>