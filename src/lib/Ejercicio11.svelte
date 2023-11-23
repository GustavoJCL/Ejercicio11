<script lang="ts">
// import Icon from 'svelte-icons-pack/Icon.svelte';
// import BiSend from "svelte-icons-pack/bi/BiSend";
	// media1: f64,
	//     variaza1: f64,
	//     k: u64,
	//     media2: f64,
	//     number_simulation: i64,
	//     specification: f64,
	//   err: f64,
	let media1: number|null ;
	let variaza1: number|null ;
	let k: number|null ;
	let media2 : number|null ;
	let number_simulation : number|null ;
	let specification : number|null ;
	let err : number|null ;
	let replicas: number|null;
</script>

<div class="flex flex-col items-center">
<h2 class="text-3xl font-bold underline text-ce">Ejercicio 11</h2>

	<div class="flex flex-row items-center m-5 space-x-6">
		<div class="order-1 w-32 h-12">
			<label class="mr-2" for="media1">Media 1:</label>
			<input type="text" class="input variant-form-material" bind:value={media1} id="media1" />
		</div>
		<div class="order-1 w-32 h-12">
			<label class="mr-2" for="variaza1">Variaza 1:</label>
			<input type="text" class="input variant-form-material" bind:value={variaza1} id="variaza1" />
		</div>
		<div class="order-1 w-32 h-12">
			<label class="mr-2" for="k">K:</label>
			<input type="text" class="input variant-form-material" bind:value={k} id="k" />
		</div>
		<div class="order-1 w-32 h-12">
			<label class="mr-2" for="media2">Media 2:</label>
			<input type="text" class="input variant-form-material" bind:value={media2} id="media2" />
		</div>
	</div>
	<div class="flex flex-row items-center m-5 space-x-6">
		<div class="order-2 w-52 h-12">
			<label class="mr-2" for="number_simulation">Number Simulation:</label>
			<input type="text" class="input variant-form-material" bind:value={number_simulation} id="number_simulation" />
		</div>
		<div class="order-2 w-32 h-12">
			<label class="mr-2" for="specification">Specification:</label>
			<input type="text" class="input variant-form-material" bind:value={specification} id="specification" />
		</div>
		<div class="order-2 w-32 h-12">
			<label class="mr-2" for="err">Error:</label>
			<input type="text" class="input variant-form-material" bind:value={err} id="err" />
		</div>
		<div class="order-2 w-52 h-12">
			<label class = "mr-2" for="replicas" >Numero de Replicas:</label>
			<input type="text" class="input variant-form-material" bind:value={replicas} id="replicas" />
		</div>
	</div>

</div>


<button type="button" class="btn variant-filled">
		<span>
		<!-- <Icon src={BiSend} /> -->
</span>
	<span>Button</span>
</button>
