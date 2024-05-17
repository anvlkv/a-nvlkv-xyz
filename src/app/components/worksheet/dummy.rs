use leptos::*;

#[component]
pub fn WorksheetDummy() -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <div class="grow-0 flex items-end flex-wrap w-full mb-6">
                <div class="shrink-0 text-2xl md:text-3xl xl:text-4xl block mr-3">
                    <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-5 md:h-7 after:content-[' ']"></div>
                </div>
                    <div class="flex justify-end grow items-end h-full">
                        <div class="border-b-2 px-2 border-slate-400 grow after:content-[' ']">
                        </div>
                        <div class="worksheet-tab block rounded-t px-4 pt-3 pb-1 ml-0 mr-px border border-slate-400 border-b-2 -mb-px border-b-transparent">
                            <div class="dummy-line rounded-sm w-24 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
                        </div>
                        <div class="worksheet-tab block rounded-t px-4 pt-3 pb-1 ml-0 mr-px border border-slate-400 border-b-2">
                            <div class="dummy-line rounded-sm w-24 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
                        </div>
                        <div class="worksheet-tab block rounded-t px-4 pt-3 pb-1 ml-0 mr-px border border-slate-400 border-b-2">
                            <div class="dummy-line rounded-sm w-24 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
                        </div>
                    </div>
            </div>
            <div class="grow w-full">
                <div class="dummy-line rounded-sm w-96 max-w-full mb-2 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
                <div class="dummy-line rounded-sm w-96 max-w-full mb-2 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
                <div class="dummy-line rounded-sm w-96 max-w-full mb-2 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
                <div class="dummy-line rounded-sm w-64 max-w-full mb-2 bg-stone-300 dark:bg-stone-700 h-4 after:content-[' ']"></div>
            </div>
        </div>
    }
}
