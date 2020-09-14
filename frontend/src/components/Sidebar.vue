<template>
  <div>
    <div v-if="results.t === 'err'"
      class="info-container">
      <div v-if="results.error === 0"
        class="search-error">
        Your search query is invalid.
      </div>
      <div v-if="results.error === 1"
        class="search-error">
        There seems to be a network error.
      </div>
      <div v-if="results.error === 2"
        class="search-error">
        Aiya! We have no resultss.
      </div>
      <div>
        <p><b>Tips on how to search:</b></p>
        <ul>
          <li>
            Space between syllables (nei hou, not neihou)
          </li>
          <li>
            ? to match a single syllable or character
          </li>
        </ul>
      </div>
    </div>
    <div v-if="results.t === 'ok'"
      class="word-list">
      <WordEntryPreview
        v-for="word in results.inner"
        :key="word.id"
        :word-entry="word"
        @click="onClick" />
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import WordEntryPreview from './WordEntryPreview.vue'

export default Vue.extend({
  props: {
    results: Object
  },
  components: {
    WordEntryPreview
  },
  methods: {
    onClick: function (id: number) {
      this.$emit('click', id)
    }
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="scss" scoped>
.info-container {
  padding-top: 1rem;
}
.search-error {
  font-weight: 500;
  font-size: 1.5rem;
  margin: 0.5rem 0;
}
</style>
