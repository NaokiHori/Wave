import os
import sys

project = "Wave"
author = "Naoki Hori"
copyright = f"2024, {author}"

mathjax_path = "https://cdn.jsdelivr.net/npm/mathjax@2/MathJax.js?config=TeX-AMS-MML_HTMLorMML"

macros = dict()
macros["tder"] = ["{\\frac{d #1}{d #2}}", 2]
macros["pder"] = ["{\\frac{\\partial #1}{\\partial #2}}", 2]
macros["pos"] = ["{\\eta_{#1}^{#2}}", 2]
macros["vel"] = ["{\\zeta_{#1}^{#2}}", 2]
macros["kpos"] = ["{H_{#1}^{#2}}", 2]
macros["kvel"] = ["{Z_{#1}^{#2}}", 2]
macros["force"] = ["{f_{#1}}", 1]
macros["dimvar"]= ["{\\tilde{#1}}", 1]
macros["ave"] = ["{\\overline{#1}}", 1]
macros["dif"] = ["{\\frac{\\delta #1}{\\delta #2}}", 2]
macros["vat"] = ["{\\left. #1 \\right|_{#2}}", 2]
macros["mysum"] = ["\\sum_{{#1} = {#2}}^{#3}", 3]
macros["myint"] = ["{\\int_{#1}^{#2} {#3} {d#4}}", 4]
macros["wavenumber"] = ["\\frac{\\pi {#1} #2}{#3}", 3]
macros["intfactor"] = ["\\epsilon_k \\left( #1 \\right)", 1]
macros["dstii"] = ["{\\mathcal{S} \\left[ #1 \\right]}", 1]
macros["dstiii"] = ["{\\mathcal{S}^{-1} \\left[ #1 \\right]}", 1]
mathjax3_config = {"TeX": {"Macros": macros}}

html_theme = "alabaster"
html_theme_options = {
        "description": "A simple energy-consistent wave equation solver",
        "fixed_sidebar": "false",
        "github_banner": "false",
        "github_button": "true",
        "github_count": "true",
        "github_repo": "Wave",
        "github_type": "star",
        "github_user": "NaokiHori",
        "navigation_with_keys": "true",
        "nosidebar": "false",
        "page_width": "95vw",
        "show_powered_by": "true",
        "show_related": "false",
        "show_relbars": "false",
        "sidebar_collapse": "true",
        "sidebar_includehidden": "false",
        "sidebar_width": "25vw",
        "gray_1": "#bbb",
        "gray_2": "#111",
        "gray_3": "#555",
        "pink_1": "#033",
        "pink_2": "#055",
        "pink_3": "#2ad3d3",
        "base_bg": "#000",
        "base_text": "#fff",
        "hr_border": "#4e4b49",
        "body_text": "#c1bcb6",
        "footer_text": "#777",
        "link": "#ffb494",
        "link_hover": "#92beff",
        "sidebar_text": "#aaa",
        "sidebar_link_underscore": "#666",
        "sidebar_search_button": "#333",
        "sidebar_list": "#fff",
        "anchor": "#222",
        "anchor_hover_bg": "#151515",
        "table_border": "#777",
        "admonition_border": "#333",
        "note_border": "#333",
        "seealso_border": "#333",
        "tip_border": "#333",
        "hint_border": "#333",
        "important_border": "#333",
        "highlight_bg": "#050c17",
        "xref_border": "#000",
        "xref_bg": "#040404",
        "admonition_xref_border": "#050505",
        "footnote_bg": "#020202",
        "narrow_sidebar_bg": "#ccc",
        "narrow_sidebar_fg": "#000",
        "viewcode_target_bg": "#002",
        "code_bg": "#130f0c",
        "code_text": "#ddd",
        "code_hover": "#111",
        "code_highlight": "#003",
}

html_static_path = ["_static"]

