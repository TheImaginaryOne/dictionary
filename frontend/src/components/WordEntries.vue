<template>
  <div class="word-entries">
    <div class="word-entries-characters">{{ wordEntries.traditional }} ({{ wordEntries.simplified }})</div>
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
.word-entries-characters {
  font-size: 1.75rem;
  padding-top: .75rem;
}
.word-entries {
  border-bottom: 1px solid #bbb;
  padding-bottom: 0.5rem;
}
</style>
