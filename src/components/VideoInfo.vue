<template>
    <div class="break-words flex flex-col gap-2">
        <h1 class="text-lg">{{ videoTitle }}</h1>
        <img :src="videoImg" :alt="videoTitle" />
        <p class=" ">{{ videoDescription }}</p>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, PropType } from 'vue';

export default defineComponent({
    name: 'VideoDisplay',
    props: {
        videoTitle: {
            type: String as PropType<string>,
            required: true,
        },
        videoImg: {
            type: String as PropType<string>,
            required: true,
        },
        videoDescription: {
            type: String as PropType<string>,
            required: true,
        },
    },
    setup(props) {
        // Using refs to handle props if needed
        const title = ref(props.videoTitle);
        const img = ref(props.videoImg);
        const description = ref(props.videoDescription);

        return {
            title,
            img,
            description,
        };
    },
});
</script>