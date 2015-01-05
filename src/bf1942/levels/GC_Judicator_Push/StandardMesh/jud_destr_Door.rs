subshader "jud_destr_Door_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.243137 0.258824 0.505882;
	texture "texture/buildings/judicator/jud_door01";
}

subshader "jud_destr_Door_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.8;
	texture "texture/buildings/hoth/glow_ID_4";
}

