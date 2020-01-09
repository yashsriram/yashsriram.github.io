import os

from mako.template import Template

TEMPLATEX_DIR = './templatex/'
TEX_DIR = './tex/'
MAIN_FILE_NAME = 'main.templatex'
SUB_FILE_PATHS = []


def change_extension_to_tex(original_path):
    pre, ext = os.path.splitext(original_path)
    return pre + '.tex'


def add_sub_file(sub_file):
    global SUB_FILE_PATHS
    SUB_FILE_PATHS.append(sub_file)
    return '\\subfile{%s}' % (change_extension_to_tex(sub_file))


def add_statement(_id, description, significance, _type):
    if significance.isspace():
        latex_significance = '{\\color{red} No significance?}'
    else:
        latex_significance = '\\textbf{Significance}:%s' % significance
    latex_statement = '\\begin{%s}\n' \
                      '\\label{def:%s}\n' \
                      '\\textbf{%s}\\par\n' \
                      '%s\n' \
                      '%s\\par\n' \
                      '\end{%s}\n' % (_type, _id, _id, description, latex_significance, _type)
    return latex_statement


def add_definition(_id, description, significance):
    return add_statement(_id, description, significance, 'definition')


def save_as_tex(rendered, output_path):
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    output_file = open(output_path, 'w')
    output_file.write(rendered)
    output_file.close()


main_template = Template(filename='{}{}'.format(TEMPLATEX_DIR, MAIN_FILE_NAME))
output = main_template.render(add_sub_file=add_sub_file)
save_as_tex(output, '{}{}'.format(TEX_DIR, change_extension_to_tex(MAIN_FILE_NAME)))

for sub_file_path in SUB_FILE_PATHS:
    template = Template(filename='{}{}'.format(TEMPLATEX_DIR, sub_file_path))
    output = template.render(add_definition=add_definition)
    save_as_tex(output, '{}{}'.format(TEX_DIR, change_extension_to_tex(sub_file_path)))
