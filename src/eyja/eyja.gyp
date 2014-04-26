{
	'targets': [
		{
			## eyja console app
			'target_name': 'eyja',
			'type': 'executable',
			'msvs_guid': 'D913FA4F-3B77-4D90-B6E8-7BAE70C8CEA8',
			'dependencies': [
			],
			'conditions': [],
			'include_dirs': [
			],
			'msvs_precompiled_header': './stdafx.h',
    		'msvs_precompiled_source': './stdafx.cpp',
			'sources': [
				'./main.cpp',
				'./stdafx.h',
				'./stdafx.cpp',
				'./XGetopt.h',
				'./XGetopt.cpp'
			]
		}
	]
}