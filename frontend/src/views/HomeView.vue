<script>
import NavBar from '@/components/NavBar.vue';
export const HomeView = {
  components: {
    NavBar
  },
  data () {
    return {
      is_login: false,
      role: ''
    }
  },
  methods: {
  },
}
</script>

<template>
    <header>
    <NavBar />
    </header>
</template>
