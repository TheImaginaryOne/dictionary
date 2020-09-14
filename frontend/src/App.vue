<template>
  <div id="app">
    <div class="header-container">
      <div class="full-width header">
        <div class="title">
          粵語詞典 Cantonese Dictionary
        </div>
        <SearchForm @update:query="updateQuery" />
      </div>
    </div>
    <div class="full-width">
      <div class="sidebar">
        <Sidebar
          :results="results"
          @click="updateWordEntry" />
      </div>
      <div class="main-entries">
        <div v-if="wordEntries.t === 'ok'">
          <div class="word-entries-characters">{{ wordEntries.inner.traditional }} ({{ wordEntries.inner.simplified }})</div>
          <SingleDictEntries v-for="(entries, dictId) in wordEntries.inner.entries"
            :key="dictId"
            :dictionaryId="dictId"
            :entries="entries" />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import SearchForm from './components/SearchForm.vue'
import SingleDictEntries from './components/SingleDictEntries.vue'
import Sidebar from './components/Sidebar.vue'
/* eslint-disable camelcase */

enum SearchErrorType {
  InvalidQuery,
  NetworkError,
  NoResults,
  EmptyQuery,
}
enum WordErrorType {
  Error,
  EmptyQuery,
}
interface Ok<T> {
  t: 'ok';
  inner: T;
}
interface Err<T> {
  t: 'err';
  error: T;
}
class Word {
  word_id = 0 // dummy to satisfy Typescript
}

type Result<T, E> = Ok<T> | Err<E>;
type SearchResult = Result<Word[], SearchErrorType>;
type WordFetchResult = Result<never[], WordErrorType>;

async function getWordEntries (wordId: number): Promise<WordFetchResult> {
  try {
    const response = await fetch(`/api/word/${wordId}`)
    const data = await response.json()
    return { t: 'ok', inner: data }
  } catch (error) {
    console.error(error)
  }
  return { t: 'err', error: WordErrorType.Error }
}

async function getSearchResults (query: string, searchType: string): Promise<SearchResult> {
  try {
    if (query === '') {
      return { t: 'err', error: SearchErrorType.EmptyQuery }
    }
    const response = await fetch(`/api/search/${searchType}/${encodeURIComponent(query)}`)
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
    SearchForm,
    SingleDictEntries,
    Sidebar
  },
  data: function () {
    return {
      results: { t: 'err', error: SearchErrorType.EmptyQuery } as SearchResult,
      wordEntries: { t: 'err', error: WordErrorType.EmptyQuery } as WordFetchResult
    }
  },
  methods: {
    // called when SearchForm emits a update:query event
    updateQuery: async function (query: string, searchType: string /** todo */) {
      this.results = await getSearchResults(query, searchType)
    },
    updateWordEntry: async function (id: number) {
      if (this.results.t === 'ok') {
        this.wordEntries = await getWordEntries(id)
      }
    }
  }
})
</script>

<style lang="scss">
@import url('https://fonts.googleapis.com/css?family=Fira+Sans:400,500,700&display=swap');
$header-height: 60px;
body, html {
  margin: 0;
  height: 100%;
}
#app {
  height: 100%;
  background-color: #eee;
}
.header-container {
  background-color: #6f8996;
  height: $header-height;
}
.full-width {
  font-family: 'Fira Sans', sans-serif;
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  line-height: 1.5;
  display: flex;

  height: calc(100% - #{$header-height});
}

.sidebar {
  flex-basis: 300px;
  height: auto;
  overflow-y: scroll;
  background-color: #fff;
}
.main-entries {
  width: 100%;
  padding-left: 16px;
  background-color: #fff;
}
.word-entries-characters {
  font-size: 1.75rem;
  padding-top: .75rem;
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
