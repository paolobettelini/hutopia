<template>
    <h2>User Registration</h2>
    <form action="/api/register" method="post">
        <label for="username">Username:</label><br>
        <input type="text" id="username" name="username"><br><br>
        <input type="submit" value="Register">
    </form>
</template>
