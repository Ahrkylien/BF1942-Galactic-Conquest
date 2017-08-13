subshader "ProbeDroid_HUD_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent true;
	blendSrc sourceAlpha;
	blendDest one;
        depthwrite false; 
	alphaTestRef 0.7;
	texture "Mods\GCMOD\Movies\probedroid";
}

