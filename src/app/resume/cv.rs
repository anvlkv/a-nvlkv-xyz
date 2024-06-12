use std::collections::HashSet;

use leptos::*;

use crate::app::state::CvEntry;

#[component]
pub fn CvView(#[prop(into)] data: Vec<CvEntry>) -> impl IntoView {
    let skills = {
        let mut skills = data
            .iter()
            .fold(HashSet::new(), |mut acc, entry| {
                acc.extend(entry.skills.clone());
                acc
            })
            .into_iter()
            .collect::<Vec<_>>();

        skills.sort();

        skills
    };

    let (selected_skills, set_selected_skills) =
        create_signal(HashSet::<String>::from_iter(skills.clone()));

    let toggle_skill = move |skill: &str, selected: bool| {
        set_selected_skills.update(|d| {
            if selected {
                d.insert(skill.to_string());
            } else {
                d.remove(skill);
            }
        });
    };

    let toggle_all = {
        let all = skills.clone();
        move |selected: bool| {
            set_selected_skills.update(|d| {
                if selected {
                    d.extend(all.clone());
                } else {
                    d.clear();
                }
            })
        }
    };

    let all_selected = {
        let len = skills.len();
        Signal::derive(move || selected_skills.get().len() == len)
    };

    view! {
        <section class="p-8 print:p-0 font-serif bg-gray-100 dark:bg-gray-800 grid grid-cols-4 auto-rows-min gap-2 rounded-lg">
            <header class="col-span-full flex gap-2 items-baseline border-b border-gray-500">
                <div class="shrink-[0.5]">
                    <h2 class="text-xl">{t!("name")}</h2>
                    <h3 class="text-lg font-thin">{t!("specialty")}</h3>
                </div>
                <p class="grow-0 border-l border-gray-500 pl-2">
                    {t!("cv.summary")}
                </p>
            </header>
            <div class="grow col-span-3">
                <h4 class="text-lg font-semibold">
                    <span>{t!("cv.experience")}{" "}</span>
                    {
                        let data = data.clone();
                        move || {
                            let selected = selected_skills.get();
                            let hidden_count = data.iter().filter(|d| d.skills.iter().all(|s| !selected.contains(s))).count();
                            if hidden_count > 0 {
                                view!{
                                    <small class="text-xs">
                                        {t!("cv.hidden", count = hidden_count)}
                                    </small>
                                }.into_view()
                            }
                            else {
                                ().into_view()
                            }
                        }
                    }
                </h4>
                <ul>
                    {move || {
                        let selected = selected_skills.get();
                        data.clone().into_iter().filter_map(|d| {
                            if d.skills.iter().any(|s| selected.contains(s)) {
                                Some(view!{
                                    <li class="my-2">
                                        <div class="flex w-full justify-between">
                                            <p class="font-bold">
                                                {d.title.clone()}
                                            </p>
                                            <p class="font-thin">
                                                {d.start_date}
                                                {" – "}
                                                {d.end_date.unwrap_or_else(|| t!("cv.present").to_string())}
                                            </p>
                                        </div>
                                        <div class="font-italic font-thin">
                                            {d.org_name.clone()}
                                        </div>
                                        <ul class="list-disc pl-4">
                                            {d.description.lines().map(|l| view!{
                                                <li>{l.to_string()}</li>
                                            }).collect_view()}
                                        </ul>
                                    </li>
                                })
                            }
                            else {
                                None
                            }
                        }
                    ).collect_view()}}
                </ul>
            </div>
            <div class="border-l border-gray-500 pl-2">
                <label class="flex items-center justify-between w-full mb-2">
                    <h4 class="text-lg">{t!("cv.skills")}</h4>
                    <input
                        attr:type="checkbox"
                        class="print:hidden"
                        name="all-skills"
                        checked={all_selected}
                        on:change={move |e| {
                            let val = event_target_checked(&e);
                            toggle_all(val);
                        }}
                    />
                </label>
                <ul class="list-disc pl-4 mb-2">
                    {
                        let skills = skills.clone();
                        move || skills.iter().filter_map(|s| {
                            let skill = s.clone();
                            if selected_skills.get().contains(&skill) {
                                Some(view!{
                                    <li>
                                        <label class="flex items-center justify-between w-full">
                                            <span>{skill.clone()}</span>
                                            <input
                                                attr:type="checkbox"
                                                class="print:hidden"
                                                name={skill.clone()}
                                                checked=true
                                                on:change={
                                                    let skill = skill.clone();
                                                    move |e| {
                                                        let val = event_target_checked(&e);
                                                        toggle_skill(&skill, val);
                                                    }
                                                }
                                            />
                                        </label>
                                    </li>
                                })
                            }
                            else {
                                None
                            }
                        }).collect_view()
                    }
                </ul>
                <Show when={
                    let skills = skills.clone();
                    move || skills.iter().any(|s| !selected_skills.get().contains(s))
                }>
                    <h4 class="text-base mb-2">{t!("cv.other_skills")}</h4>
                    <ul class="list-disc pl-4 text-sm font-thin">
                        {
                            let skills = skills.clone();
                            move || {
                            let selected = selected_skills.get();
                            skills.iter().filter_map(|s| if !selected.contains(s) {
                                let skill = s.clone();
                                Some(view!{
                                    <li>
                                        <label class="flex items-center justify-between w-full">
                                            <span>{skill.clone()}</span>
                                            <input
                                                attr:type="checkbox"
                                                class="print:hidden"
                                                name={skill.clone()}
                                                checked=false
                                                on:change={
                                                    let skill = skill.clone();
                                                    move |e| {
                                                        let val = event_target_checked(&e);
                                                        toggle_skill(&skill, val);
                                                    }
                                                }
                                            />
                                        </label>
                                    </li>
                                })
                            } else {
                                None
                            }).collect_view()
                        }}
                    </ul>
                </Show>
            </div>
        </section>
    }
}

#[component]
pub fn CvDummyView() -> impl IntoView {
    view! {
        <section class="sheet padding-10mm font-serif bg-gray-100 dark:bg-gray-800 grid items-baseline grid-cols-4 auto-rows-min gap-2 rounded-lg">
            <header class="col-span-full flex max-md:flex-col gap-2 flex-wrap items-baseline border-b border-gray-500">
                <div class="shrink">
                    <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-5 md:h-7 mb-2 after:content-[' ']"></div>
                    <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-5 md:h-7 mb-2 after:content-[' ']"></div>
                </div>
                <p class="shrink-0 border-l border-gray-500 pl-2">
                    <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-3 md:h-5 mb-1 after:content-[' ']"></div>
                </p>
            </header>
            <div class="grow col-span-3">
                <h4 class="text-lg font-semibold">
                    <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-5 md:h-7 mb-2 after:content-[' ']"></div>
                </h4>
                <ul>
                    {(0..=3).map(|_| view!{
                        <li class="my-2">
                            <div class="flex w-full">
                                <p class="font-bold">
                                    <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-3 md:h-5 mb-1 after:content-[' ']"></div>
                                </p>
                                <p>
                                    <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-3 md:h-5 mb-1 after:content-[' ']"></div>
                                </p>
                            </div>
                            <div>
                                <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-5 md:h-7 mb-2 after:content-[' ']"></div>
                            </div>
                            <ul class="list-disc pl-4">
                                <li>
                                    <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-3 md:h-5 mb-1 after:content-[' ']"></div>
                                </li>
                                <li>
                                    <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-3 md:h-5 mb-1 after:content-[' ']"></div>
                                </li>
                                <li>
                                    <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-3 md:h-5 mb-1 after:content-[' ']"></div>
                                </li>
                            </ul>
                        </li>
                    }).collect_view()}
                </ul>
            </div>
            <div class="border-l border-gray-500 pl-2">
                <label class="flex items-center justify-between w-full">
                    <h4 class="text-lg">
                        <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-3 md:h-5 mb-1 after:content-[' ']"></div>
                    </h4>
                </label>
                <ul class="list-disc pl-4">
                    {(0..20).map(|_| view!{
                        <li>
                            <span>
                                <div class="dummy-line rounded-sm w-64 bg-stone-300 dark:bg-stone-700 h-3 md:h-5 mb-1 after:content-[' ']"></div>
                            </span>
                        </li>
                    }).collect_view()}
                </ul>
            </div>
        </section>
    }
}
