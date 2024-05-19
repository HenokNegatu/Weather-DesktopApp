<script lang="ts">
  import "./style/style.scss";
  import Cloud from "phosphor-svelte/lib/Cloud";
  import Thermometer from "phosphor-svelte/lib/Thermometer";
  import DropHalfBottom from "phosphor-svelte/lib/DropHalfBottom";
  import Wind from "phosphor-svelte/lib/Wind";
  import Gauge from "phosphor-svelte/lib/Gauge";
  import X from "phosphor-svelte/lib/X";
  import MagnifyingGlass from "phosphor-svelte/lib/MagnifyingGlass";
  import Sun from "phosphor-svelte/lib/Sun";
  import Moon from "phosphor-svelte/lib/Moon";

  import img from "./assets/images/mist.png";
  
  let term = "";
  const today = new Date();
  const f = Intl.DateTimeFormat("en-us", {
    dateStyle: "full",
    timeStyle: "medium",
  });

  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let weatherData = {
    location: {
      name: "",
      region: "",
      country: "",
      lat: "",
      lon: "",
      localtime: "",
    },
    current: {
      condition: {
        icon: "",
        text: "",
      },
      is_day: "",
      humidity: "",
      pressure_in: "",
      pressure_mb: "",
      temp_c: "",
      temp_f: "",
      wind_dir: "",
      wind_kph: "",
      wind_mph: "",
    },
  };

  const fetchWeather = async (e: any) => {
    e.preventDefault();
    try {
      const result: string = await invoke("fetch_data", {address: term});
      console.log(result);
      weatherData = await JSON.parse(result);
      console.log(weatherData);
    } catch (e) {
      console.log(e);
    }
  };

  onMount(async ()=>{
    term = 'addis ababa'
    try {
      const result: string = await invoke("fetch_data", {address: term});
      console.log(result);
      weatherData = await JSON.parse(result);
      console.log(weatherData);
    } catch (e) {
      console.log(e);
    }
  })
  //theme
  let theme = "light";

  const toggleTheme = () => {
    theme = theme === "light" ? "dark" : "light";
  };
</script>

<main class={`container ${theme}`}>
  <button class="theme-btn" on:click={toggleTheme}
    >{#if theme === "light"}
      <Sun color="#000" weight="bold" size={30} />
    {:else}
      <Moon color="aliceblue" weight="bold" size={30} />{/if}</button
  >
  <form class="form" on:submit={fetchWeather}>
    <button type="submit"
      ><MagnifyingGlass color="aliceblue" weight="bold" size={30} /></button
    >
    <input type="text" placeholder="Addis Ababa" bind:value={term} required/>
    {#if term != ""}
      <button on:click={() => (term = "")}
        ><X color="red" weight="bold" size={25} /></button
      >
    {/if}
  </form>
  <div class="main-icon">
    <img src={img} alt="cloud icon" />
    <div class="info">
      <p>{weatherData.location["name"]}</p>
      <p class="date">{weatherData.current.condition["text"]}</p>
      <p class="date">{f.format(today)}</p>
      <img src={weatherData.current.condition["icon"]} alt="cloud icon" />
    </div>
  </div>

  <div class="search-meta">
    <p>local time: {weatherData.location["localtime"]}</p>
    <p>region: {weatherData.location["region"]}</p>
    <p>country: {weatherData.location["country"]}</p>
    
    <p>lat: {weatherData.location["lat"]}</p>
    <p>lon: {weatherData.location["lon"]}</p>

  </div>

  <div class="infos">
    <div class="elements">
      <div class="header">
        <Thermometer color="aliceblue" weight="bold" size={40} class="icon2" />
        <p>Temprature</p>
      </div>
      <div>
        <p class="fetched-data">{weatherData.current["temp_c"]}&deg;C</p>
        <p class="fetched-data">{weatherData.current["temp_f"]}&deg;F</p>
      </div>
    </div>

    <div class="elements">
      <div class="header">
        <DropHalfBottom
          color="aliceblue"
          weight="bold"
          size={40}
          class="icon2"
        />
        <p>Humidity</p>
      </div>
      <div>
        <p class="fetched-data">{weatherData.current["humidity"]}%</p>
      </div>
    </div>
    <div class="elements">
      <div class="header">
        <Gauge color="aliceblue" weight="bold" size={40} class="icon2" />
        <p>Pressure</p>
      </div>
      <div>
        <p class="fetched-data">{weatherData.current["pressure_in"]}inHg</p>
        <p class="fetched-data">{weatherData.current["pressure_mb"]}mb</p>
      </div>
    </div>

    <div class="elements">
      <div class="header">
        <Wind color="aliceblue" weight="bold" size={40} class="icon2" />
        <p>Wind Speed</p>
      </div>
      <div>
        <p class="fetched-data">{weatherData.current["wind_dir"]}</p>
        <p class="fetched-data">{weatherData.current["wind_kph"]}km/h</p>
        <p class="fetched-data">{weatherData.current["wind_mph"]}mph</p>

      </div>
    </div>
  </div>
</main>

<style>
</style>
