<template>
  <div class="word-entry-preview"
    @click="onClick">
    <div class="word-entry-preview-characters">{{ wordEntry.traditional }} ({{ wordEntry.simplified }})</div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({
  props: {
    wordEntry: Object
  },
  methods: {
    onClick: function () {
      this.$emit('click', this.wordEntry.word_id)
    }
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="scss" scoped>
.word-entry-preview-characters {
  font-size: 1.5rem;
}
.word-entry-preview {
  border-bottom: 1px solid #bbb;
  padding: 0.5rem 0.5rem;
  &:hover {
    background-color: #eee;
    cursor: pointer;
  }
}
</style>
