subshader "R2D2xHud_M1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.7;
        depthWrite false;
	texture "Mods\GCMOD\Movies\R2D2xHud";
}

