<template>
  <div class="single-dict-entries">
    <div class="dict-name">
      <span>{{ d(dictionaryId) }}</span>
    </div>
    <div v-for="(entry, i) in entries" :key="i" class="single-entry-content">
      <div class="pronunciations">
        <span class="pronunciation-item" v-for="(pronunciations, j) in entry.pronunciations" :key="j">{{e(j)}} {{join(pronunciations)}}</span>
      </div>
      <div>
        <ol class="definitions">
          <li v-for="(def, j) in splitDefs(entry.definitions)" :key="j">{{ def }}</li>
        </ol>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'

const a: { [key: string]: string; } = { '0': 'CC-CEDICT', '1': 'CC-Canto' }
const b: { [key: string]: string; } = { '0': '國', '1': '粵' }

export default Vue.extend({
  props: {
    dictionaryId: String,
    entries: Array
  },
  methods: {
    d: function (dictionaryId: string) {
      return a[dictionaryId]
    },
    e: function (pronunciationType: string) {
      return b[pronunciationType]
    },
    join: function (pronunciations: string[]) {
      return pronunciations.join('/')
    },
    splitDefs: function (definitions: string) {
      // `this` points to the vm instance
      return definitions.split('|').slice(1, -1)
    }
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="scss" scoped>
.single-dict-entries {
  padding-bottom: 0.5rem
}
.dict-name {
  font-weight: bold;
  padding: 0.25rem 0;
}
.pronunciations {
  color: #888;
};
.pronunciation-item {
  padding-right: .5rem;
}
.definitions {
  margin: 0;
  padding-left: 2rem;
}
.single-entry-content {
  padding-left: 1rem;
}
</style>
