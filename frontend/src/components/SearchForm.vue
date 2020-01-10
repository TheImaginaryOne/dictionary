<template>
  <div class="search-form">
    <div class="nav-item">
      <select class="nav-select">
        <option value="jyutping">Jyutping (Cantonese)</option>
        <option value="pinyin">Pinyin (Mandarin)</option>
        <option value="characters">Characters</option>
        <option value="english">English</option>
      </select>
    </div>
    <div class="nav-item">
      <input class="nav-input" v-model="query"/>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'

export default Vue.extend({
  data: function () {
    return {
      query: ''
    }
  },
  watch: {
    // when query is updated
    query: function () {
      this.updateQuery()
    }
  },
  methods: {
    updateQuery: function () {
      // emit event
      this.$emit('update:query', this.query)
    }
  }
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="scss">
.search-form {
  display: flex;
};
.nav-input {
  border: none;
  border-radius: 0.25rem;
  padding: 0.5rem;
  line-height: 1.25;
  font-size: 1rem;
  font-family: 'Fira Sans';
  background-color: #fff;
}
.nav-select {
  border: none;
  border-radius: 0.25rem;
  background-color: #c8dbe9;
  padding: 0.5rem;
  line-height: 1.25;
  font-size: 1rem;
  appearance: none;
  font-family: 'Fira Sans';
  padding-right: 1.5rem;

  background-image: url("data:image/svg+xml,%3Csvg width='24' height='16' viewBox='0 0 24 16' xmlns='http://www.w3.org/2000/svg'%3E%3Cpolygon fill='%23000' points='12 1 9 6 15 6' /%3E%3Cpolygon fill='%23000' points='12 13 9 8 15 8' /%3E%3C/svg%3E%0A");
  background-position: 100% 50%;
  background-repeat: no-repeat;
}
</style>
