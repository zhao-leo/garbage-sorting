<template>
    <div v-if="visible" class="custom-dialog">
      <div class="dialog-content">
        <h3>{{ title }}</h3>
        <p>{{ message }}</p>
      </div>
    </div>
  </template>
  
  <script>
  import { ref, onMounted } from 'vue';
  
  export default {
    props: {
      title: {
        type: String,
        default: '通知'
      },
      message: {
        type: String,
        required: true
      },
      duration: {
        type: Number,
        default: 5000
      }
    },
    setup(props, { emit }) {
      const visible = ref(true);
      
      onMounted(() => {
        setTimeout(() => {
          visible.value = false;
          emit('close');
        }, props.duration);
      });
      
      return { visible };
    }
  }
  </script>
  
  <style scoped>
  .custom-dialog {
    position: fixed;
    top: 20px;
    right: 20px;
    background: white;
    border-radius: 4px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    padding: 16px;
    z-index: 9999;
    max-width: 320px;
  }
  
  .dialog-content h3 {
    margin-top: 0;
    margin-bottom: 8px;
  }
  
  .dialog-content p {
    margin: 0;
  }
  </style>