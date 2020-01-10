<template>
  <div id="app">
    <div class="header-container">
      <div class="full-width header">
        <div class="title">粵語詞典 Cantonese Dictionary</div>
        <SearchForm @update:query="updateQuery"/>
      </div>
    </div>
    <div class="full-width">
      <WordEntries v-for="word in words" :key="word.id" :word-entries="word"/>
    </div>
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
@import url('https://fonts.googleapis.com/css?family=Fira+Sans:400,500,700&display=swap');
.full-width {
  font-family: 'Fira Sans', sans-serif;
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  line-height: 1.5;
}
body {
  margin: 0;
}
.header-container {
  background-color: #6f8996;
}
.header {
  display: flex;
  padding-top: .75rem;
  padding-bottom: 0.75rem;
}
.title {
  font-size: 1.5rem;
  font-weight: 500;
  padding-right: 2rem;
  color: #fff;
}
.nav-item {
  padding-right: 0.5rem;
}
</style>
