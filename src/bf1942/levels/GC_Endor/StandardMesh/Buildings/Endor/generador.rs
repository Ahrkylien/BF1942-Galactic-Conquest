subshader "generador_Material0" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/Buildings/Endor/text_puerta";
}

subshader "generador_Material1" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/Buildings/Endor/text_gen";
}

subshader "generador_Material2" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/Buildings/Endor/bunker_floor";
}

subshader "generador_Material3" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/Buildings/Endor/bunker_lights";
}

subshader "generador_Material4" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	transparent true;
	twosided true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.7;
	texture "Mods\GCMOD\Movies\generator_fx";
}

