<template>
    <nav class="navbar navbar-expand-lg bg-body-tertiary">
  <div class="container-fluid">
    <a class="navbar-brand">MB</a>
    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
      <span class="navbar-toggler-icon"></span>
    </button>
    <div class="collapse navbar-collapse me-2" id="navbarNav">
      <ul class="navbar-nav">
            <li class="nav-item" v-if="!is_login">
                <router-link class="nav-link" to="/signup">Register</router-link>
            </li>
            <li class="nav-item" v-if="!is_login">
                <router-link class="nav-link" to="/login">Login</router-link>
            </li>
            <li class="nav-item" v-if="role === 'admin'">
                <router-link class="nav-link" to="/admin">Admin</router-link>
            </li>
            <li class="nav-item" v-if="is_login && (role === 'creator' || role === 'admin')">
                <router-link class="nav-link" to="/upload-music">Upload</router-link>
            </li>
            <li class="nav-item" v-if="is_login">
                <router-link class="nav-link" to="/profile">Profile</router-link>
            </li>
            <li class="nav-item" v-if="is_login">
                <button class="nav-link" @click='logout'>Logout</button>
            </li>
      </ul>
    </div>
  </div>
</nav>
</template>

<script>
export default {
  components: {

  },
};
</script>

<style>

</style>