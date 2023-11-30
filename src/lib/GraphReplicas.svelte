<script lang="ts">
	import type { DataReplicas } from './interfaces';
	import { Scatter } from 'svelte-chartjs';
	import {
		Chart as ChartJS,
		Title,
		Tooltip,
		Legend,
		LineElement,
		CategoryScale,
		LinearScale,
		PointElement
	} from 'chart.js';
	ChartJS.register(Title, Tooltip, Legend, LineElement, CategoryScale, LinearScale, PointElement);
	// export let number_replica: number | null = 1;
	export let dataReplicas: DataReplicas;

	let keyScatter = Object.keys(dataReplicas).map((key) => Number(key));
	console.log('heil yazov');
	console.log(keyScatter);
	// console.log(dataReplicas[keyScatter[3]]);
	// console.log(Object.values(dataReplicas[3]));

	let datasetsScatter = keyScatter.map((key: number) => ({
		label: `Replica ${key}`,
		data: dataReplicas[key].map((item: number, index: number) => ({
			x: index,
			y: item
		})),
		backgroundColor: 'rgba(75, 192, 192, 0.2)',
		borderColor: 'rgba(75, 192, 192, 1)',
		borderWidth: 1
	}));
	console.log(datasetsScatter);
	let data = {
		labels: ['Scatter'],
		datasets: datasetsScatter
	};
	console.log(data);

	let options = {
		responsive: true,
		scales: {
			x: {
				beginAtZero: true,
				title: {
					display: true,
					text: 'Index'
				}
			},
			y: {
				beginAtZero: true,
				title: {
					display: true,
					text: 'Value'
				}
			}
		}
	};
</script>

<Scatter {data} {options} />
