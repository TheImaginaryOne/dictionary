<template>
  <div id="app">
    <div class="header-container">
      <div class="full-width header">
        <div class="title">粵語詞典 Cantonese Dictionary</div>
        <SearchForm @update:query="updateQuery"/>
      </div>
    </div>
    <div class="full-width">
      <div v-if="result.t === 'err'" class="info-container">
        <div v-if="result.error === 0" class="search-error">
          Your search query is invalid.
        </div>
        <div v-if="result.error === 1" class="search-error">
          There seems to be a network error.
        </div>
        <div v-if="result.error === 2" class="search-error">
          Aiya! We have no results.
        </div>
        <div>
          <p><b>Tips on how to search:</b></p>
          <ul>
            <li>Space between syllables (nei hou, not neihou)</li>
            <li>? to match a single syllable or character</li>
          </ul>
        </div>
      </div>
      <div v-if="result.t === 'ok'">
        <WordEntries v-for="word in result.inner" :key="word.id" :word-entries="word"/>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import SearchForm from './components/SearchForm.vue'
import WordEntries from './components/WordEntries.vue'

enum SearchErrorType {
  InvalidQuery,
  NetworkError,
  NoResults,
  EmptyQuery,
};
interface SearchOk {
  t: 'ok';
  inner: never[];
}
interface SearchError {
  t: 'err';
  error: SearchErrorType;
}

type SearchResult = SearchOk | SearchError;

async function getResults (query: string): Promise<SearchResult> {
  try {
    if (query === '') {
      return { t: 'err', error: SearchErrorType.EmptyQuery }
    }
    const response = await fetch('/api/search/jyutping/' + encodeURIComponent(query))
    const data = await response.json()
    if (data.message) {
      return { t: 'err', error: SearchErrorType.InvalidQuery }
    }
    if (data.length > 0) {
      return { t: 'ok', inner: data }
    } else {
      return { t: 'err', error: SearchErrorType.NoResults }
    }
  } catch (error) {
    console.error(error)
  }
  return { t: 'err', error: SearchErrorType.NetworkError }
}

export default Vue.extend({
  components: {
    WordEntries,
    SearchForm
  },
  data: function () {
    return {
      result: { t: 'err', error: SearchErrorType.EmptyQuery } as SearchResult
    }
  },
  methods: {
    // called when SearchForm emits a update:query event
    updateQuery: async function (query: string) {
      this.result = await getResults(query)
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
.info-container {
  padding-top: 1rem;
}
.search-error {
  font-weight: 500;
  font-size: 1.5rem;
  margin: 0.5rem 0;
}
</style>
