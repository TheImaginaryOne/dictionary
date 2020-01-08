<template>
  <div id="app">
    <SearchForm @update:query="updateQuery"/>
    <WordEntries v-for="word in words" :key="word.id" :word-entries="word"/>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import SearchForm from './components/SearchForm.vue'
import WordEntries from './components/WordEntries.vue'

async function getResults (query: string) {
  try {
    if (query === '') {
      return {}
    }
    const response = await fetch('/api/search/jyutping/' + query)
    const data = await response.json()
    return data
  } catch (error) {
    console.error(error)
  }
}

export default Vue.extend({
  components: {
    WordEntries,
    SearchForm
  },
  data: function () {
    return {
      words: []
    }
  },
  methods: {
    // called when SearchForm emits a update:query event
    updateQuery: async function (query: string) {
      this.words = await getResults(query)
    }
  }
})
</script>

<style lang="scss">
@import url('https://fonts.googleapis.com/css?family=Fira+Sans:400,700&display=swap');
#app {
  font-family: 'Fira Sans', sans-serif;
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  line-height: 1.5;
}
</style>
