<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import Icon from 'svelte-icons-pack';
	import AiOutlineSend from 'svelte-icons-pack/ai/AiOutlineSend';
	import type { TableReplicas, TableDataCollection, DataReplicas, TableData } from './interfaces';
	import { Table, tableMapperValues } from '@skeletonlabs/skeleton';
	import type { TableSource } from '@skeletonlabs/skeleton';
	import GraphReplicas from './GraphReplicas.svelte';
	// media1: f64,
	//     variaza1: f64,
	//     k: u64,
	//     media2: f64,
	//     number_simulation: i64,
	//     specification: f64,
	//   err: f64,
	let media1: string;
	let variaza1: string;
	let k: string;
	let media2: string;
	let number_simulation: string;
	let specification: string;
	let err: string;
	let replicas: string;

	let disableButton = true;
	let showResult = false;

	let returnReplicas: TableReplicas = {};
	let sourceDataTable: TableDataCollection[] = [];
	let number_replica: number | null = 1;
	let tableSource: TableSource;
	let dataReplicas: DataReplicas = {};

	console.log('esquizo');
	let ejercicio11 = async () => {
		returnReplicas = (await invoke('ejercicio_11', {
			media1: Number(media1),
			variaza1: Number(variaza1),
			k: Number(k),
			media2: Number(media2),
			number_simulation: Number(number_simulation),
			specification: Number(specification),
			err: Number(err),
			replicas: Number(replicas)
		})) as TableReplicas;
		showTable();

		console.log(returnReplicas);
		console.log(returnReplicas[1][1]);

		// if ( number_replica!=0){
		// console.log(returnReplicas[number_replica]);
		// }
	};

	type Validation = {
		value: string;
		validate: (value: string) => boolean;
	};

	let validate_data = () => {
		let validations: Validation[] = [
			{ value: media1, validate: isNonNegative },
			{ value: variaza1, validate: isNonNegative },
			{ value: k, validate: isInteger },
			{ value: media2, validate: isNonNegative },
			{ value: number_simulation, validate: isInteger },
			{ value: specification, validate: isNonNegative },
			{ value: err, validate: isNonNegative },
			{ value: replicas, validate: isInteger }
		];

		disableButton = !validations.every(({ value, validate }) => validate(value));
		console.log(disableButton);
		console.log('esquizo');
	};

	let isNonNegative = (value: string): boolean => {
		let number = parseFloat(value);
		return !isNaN(number) && number >= 0;
	};

	let isInteger = (value: string): boolean => {
		let number = parseFloat(value);
		return Number.isInteger(number);
	};

	let showTable = () => {
		if (returnReplicas[0] != null) return;

		showResult = true;
		console.log(showResult);

		console.log('heil');
		sourceDataTable = Object.values(returnReplicas[Number(number_replica)]);
		console.log(sourceDataTable);
		tableSource = {
			head: [
				'Medida 1',
				'Medida 2',
				'Longitud total',
				'ei',
				'es',
				'Coeficiente de capacidad',
				'Coeficiente de capacidad k',
				'Estado de la barra',
				'Probabilidad'
			],
			// body: tableMapperValues(sourceData, ['name', 'symbol', 'weight']),
			body: tableMapperValues(sourceDataTable, [
				'length1',
				'length2',
				'total_length',
				'ei',
				'es',
				'coeficient_capacity',
				'coeficient_capacity_k',
				'bar_state',
				'probability'
			]),
			foot: []
		};
	};

	$: {
		console.log('hola esquizo');
		console.log(dataReplicas);
		Object.entries(returnReplicas).forEach(([key, value]: [string, TableDataCollection]) => {
			const collection = value;
			// console.log(collection);
			const values: number[] = [];
			Object.values(collection).forEach((replicaValue: TableData) => {
				// console.log(replicaKey);
				// console.log(replicaValue.probability);
				values.push(replicaValue.probability);
			});
			dataReplicas[Number(key)] = values;
		});
		console.log(dataReplicas);
		// validate_data();
	}
</script>

<div class="flex flex-col items-center">
	<h2 class="text-3xl font-bold underline text-ce">Ejercicio 11</h2>

	<div class="flex flex-row items-center m-5 space-x-6">
		<div class="order-1 w-32 h-12">
			<label class="mr-2" for="media1">Media 1:</label>
			<input
				type="text"
				class="input variant-form-material"
				bind:value={media1}
				id="media1"
				on:change={validate_data}
			/>
		</div>
		<div class="order-1 w-32 h-12">
			<label class="mr-2" for="variaza1">Variaza 1:</label>
			<input
				type="text"
				class="input variant-form-material"
				bind:value={variaza1}
				id="variaza1"
				on:change={validate_data}
			/>
		</div>
		<div class="order-1 w-32 h-12">
			<label class="mr-2" for="k">K:</label>
			<input
				type="text"
				class="input variant-form-material"
				bind:value={k}
				id="k"
				on:change={validate_data}
			/>
		</div>
		<div class="order-1 w-32 h-12">
			<label class="mr-2" for="media2">Media 2:</label>
			<input
				type="text"
				class="input variant-form-material"
				bind:value={media2}
				id="media2"
				on:change={validate_data}
			/>
		</div>
	</div>
	<div class="flex flex-row items-center m-5 space-x-6">
		<div class="order-2 w-52 h-12">
			<label class="mr-2" for="number_simulation">Number Simulation:</label>
			<input
				type="text"
				class="input variant-form-material"
				bind:value={number_simulation}
				on:change={validate_data}
				id="number_simulation"
			/>
		</div>
		<div class="order-2 w-32 h-12">
			<label class="mr-2" for="specification">Specification:</label>
			<input
				type="text"
				class="input variant-form-material"
				bind:value={specification}
				id="specification"
				on:change={validate_data}
			/>
		</div>
		<div class="order-2 w-32 h-12">
			<label class="mr-2" for="err">Error:</label>
			<input
				type="text"
				class="input variant-form-material"
				bind:value={err}
				id="err"
				on:change={validate_data}
			/>
		</div>
		<div class="order-2 w-52 h-12">
			<label class="mr-2" for="replicas">Numero de Replicas:</label>
			<input
				type="text"
				class="input variant-form-material"
				bind:value={replicas}
				id="replicas"
				on:change={validate_data}
			/>
		</div>
	</div>

	<button type="button" class="btn variant-filled" on:click={ejercicio11} disabled={disableButton}>
		<span>
			<Icon src={AiOutlineSend} />
		</span>
		<span>Calcular</span>
	</button>
</div>

{#if showResult}
	<div class="m-5">
		<label class="text-black label">
			<select class="w-72 select" bind:value={number_replica} on:change={showTable}>
				<option value={1}>Seleccionar Replicas</option>
				{#each Object.keys(returnReplicas) as keyReturnReplicas}
					<option value={keyReturnReplicas}>{keyReturnReplicas}</option>
				{/each}
			</select>
		</label>
	</div>

	<Table source={tableSource} interactive={true} />
	<p>hola esquizo</p>
	<GraphReplicas {dataReplicas} />
	<p>esquizo</p>
{/if}
