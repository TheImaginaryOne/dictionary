<template>
  <div class="word-entry-preview">
    <div class="word-entry-preview-characters">{{ wordEntries.traditional }} ({{ wordEntries.simplified }})</div>
<!--    <SingleDictEntries v-for="(entries, dictId) in wordEntries.entries"
      :key="dictId"
      :dictionaryId="dictId"
      :entries="entries"/>-->
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
// import SingleDictEntries from './SingleDictEntries.vue'

export default Vue.extend({
  props: {
    wordEntries: Object
  },
  components: {
  //  SingleDictEntries
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
}
</style>
