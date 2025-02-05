<template>
    <div class="video-container">
      <video ref="videoPlayer" autoplay loop class="background-video">
        <source src="./assets/1.mp4" type="video/mp4">
        Your browser does not support the video tag.
      </video>
      <div class="overlay-button" @click="navigateToNewPage">
        Click to navigate to a new page
      </div>
    </div>
  </template>
  
  <script>
  export default {
    methods: {
      navigateToNewPage() {
        this.$router.push('/new-page')
      }
    }
  }
  </script>
  
  <style scoped>
  .video-container {
    position: relative;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
  }
  
  .background-video {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    min-width: 90%;
    min-height: 90%;
    border-radius: 20px;
    overflow: hidden;
  }
  
  .overlay-button {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: rgba(0, 0, 0, 0);
    color: rgba(0, 0, 0, 0);
    font-size: 2em;
    cursor: pointer;
    transition: background-color 0.3s;
  }
  
  .overlay-button:hover {
    background-color: rgba(0, 0, 0, 0);
  }
  </style>