<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
import Icon from 'svelte-icons-pack';
import AiOutlineSend from "svelte-icons-pack/ai/AiOutlineSend";

import { Table, tableMapperValues } from '@skeletonlabs/skeleton';
import type { TableSource } from '@skeletonlabs/skeleton';
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

	interface TableData{
	length1: number;
	length2: number;
	total_length: number;
	ei: number;
	es: number;
	coeficient_capacity: number;
	coeficient_capacity_k: number;
	bar_state: boolean;
	probability: number;
} 
	
interface TableDataCollection {
	[key: number]: TableData;
}

interface TableReplicas{
	[key: number]: TableDataCollection;
}


	let returnReplicas: TableReplicas;
	let sourceDataTable: TableDataCollection [] = [];
	let number_replica = 0;
	let tableSource: TableSource ;

	console.log("esquizo");
	let ejercicio11 =async ()=>{
		
		returnReplicas=await invoke(
			'ejercicio_11',
			{
				media1: media1,
				variaza1: variaza1,
				k: k,
				media2: media2,
				number_simulation: number_simulation,
				specification: specification,
				err: err,
				replicas: replicas,
			}
			) as TableReplicas;

		console.log(returnReplicas);
		console.log(returnReplicas[1][1]);
		// if ( number_replica!=0){
			// console.log(returnReplicas[number_replica]);
			sourceDataTable = Object.values(returnReplicas[number_replica]);
			console.log(sourceDataTable);
			tableSource = {
				head: ['length1', 'length2', 'total_length', 'ei', 'es', 'coeficient_capacity', 'coeficient_capacity_k', 'bar_state', 'probability'],
				body: tableMapperValues(sourceDataTable, ['length1', 'length2', 'total_length', 'ei', 'es', 'coeficient_capacity', 'coeficient_capacity_k', 'bar_state', 'probability']),
				foot: []
			}
		// }
}
</script>

<div class="flex flex-col items-center">
<h2 class="text-3xl font-bold underline text-ce">Ejercicio 11</h2>

	<div class="flex flex-row items-center m-5 space-x-6">
		<div class="order-1 w-32 h-12">
			<label class="mr-2" for="media1">Media 1:</label>
			<input type="number" class="input variant-form-material" bind:value={media1} id="media1" />
		</div>
		<div class="order-1 w-32 h-12">
			<label class="mr-2" for="variaza1">Variaza 1:</label>
			<input type="number" class="input variant-form-material" bind:value={variaza1} id="variaza1" />
		</div>
		<div class="order-1 w-32 h-12">
			<label class="mr-2" for="k">K:</label>
			<input type="number" class="input variant-form-material" bind:value={k} id="k" />
		</div>
		<div class="order-1 w-32 h-12">
			<label class="mr-2" for="media2">Media 2:</label>
			<input type="number" class="input variant-form-material" bind:value={media2} id="media2" />
		</div>
	</div>
	<div class="flex flex-row items-center m-5 space-x-6">
		<div class="order-2 w-52 h-12">
			<label class="mr-2" for="number_simulation">Number Simulation:</label>
			<input type="number" class="input variant-form-material" bind:value={number_simulation} id="number_simulation" />
		</div>
		<div class="order-2 w-32 h-12">
			<label class="mr-2" for="specification">Specification:</label>
			<input type="number" class="input variant-form-material" bind:value={specification} id="specification" />
		</div>
		<div class="order-2 w-32 h-12">
			<label class="mr-2" for="err">Error:</label>
			<input type="number" class="input variant-form-material" bind:value={err} id="err" />
		</div>
		<div class="order-2 w-52 h-12">
			<label class = "mr-2" for="replicas" >Numero de Replicas:</label>
			<input type="number" class="input variant-form-material" bind:value={replicas} id="replicas" />
		</div>
	</div>
<button type="button" class="btn variant-filled" on:click={ejercicio11}>
	<span>
		<Icon src={AiOutlineSend} />
	</span>
	<span>Calcular</span>
</button>
</div>


<div>
		<div>
		<input type="number" class="input variant-form-material" bind:value={number_replica}>
	</div>
	{#if number_replica>0}
		<Table source={tableSource}/>
	{/if}

</div>
